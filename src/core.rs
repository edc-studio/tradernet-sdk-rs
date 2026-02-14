use crate::common::net_utils::NetUtils;
use crate::common::string_utils::{sign, stringify};
use crate::errors::TradernetError;
use log::{debug, warn};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Core Tradernet client that handles authentication and HTTP requests.
pub struct Core {
    /// Public API key.
    pub public: Option<String>,
    private: Option<String>,
    net: NetUtils,
}

impl Core {
    /// Base API domain used for REST and WebSocket URLs.
    pub const DOMAIN: &str = "freedom24.com";
    /// Default session time-to-live (seconds).
    pub const SESSION_TIME: u64 = 18_000;
    /// Chunk size used in batched export operations.
    pub const CHUNK_SIZE: usize = 7_000;
    /// Max number of symbols per export request.
    pub const MAX_EXPORT_SIZE: usize = 100;

    /// Creates a new [`Core`] with optional API keys.
    pub fn new(public: Option<String>, private: Option<String>) -> Result<Self, TradernetError> {
        let net = NetUtils::new(Duration::from_secs(300))?;
        if public.is_none() || private.is_none() {
            warn!("A keypair was not set. It can be generated here: {}/tradernet-api/auth-api", Self::url());
        }

        Ok(Self {
            public,
            private,
            net,
        })
    }

    /// Creates a [`Core`] from an INI config file containing the keypair.
    pub fn from_config(path: impl AsRef<Path>) -> Result<Self, TradernetError> {
        let (public, private) = load_auth_from_ini(path.as_ref())?;

        Self::new(public, private)
    }

    /// Returns the base HTTPS URL for the REST API.
    pub fn url() -> String {
        format!("https://{}", Self::DOMAIN)
    }

    /// Returns the base WSS URL for the WebSocket API.
    pub fn websocket_url() -> String {
        format!("wss://wss.{}", Self::DOMAIN)
    }

    /// Builds authentication query parameters for WebSocket connections.
    pub fn websocket_auth(&self) -> HashMap<String, String> {
        let timestamp = current_timestamp();
        let private_key = self.private.clone().unwrap_or_default();

        HashMap::from([
            ("X-NtApi-PublicKey".to_string(), self.public.clone().unwrap_or_default()),
            ("X-NtApi-Timestamp".to_string(), timestamp.clone()),
            ("X-NtApi-Sig".to_string(), sign(&private_key, &timestamp)),
        ])
    }

    /// Sends an unauthenticated `GET /api` request with a command and params.
    pub fn plain_request(&self, cmd: &str, params: Option<Map<String, Value>>) -> Result<Value, TradernetError> {
        debug!("Making a simple request to API");

        let mut message = Map::new();
        message.insert("cmd".to_string(), Value::String(cmd.to_string()));
        if let Some(params) = params {
            message.insert("params".to_string(), Value::Object(params));
        }

        let query = vec![("q".to_string(), stringify(&Value::Object(message))?)];
        let url = format!("{}/api", Self::url());

        debug!("Query: {:?}", query);
        let response = self.net.request(Method::GET, &url, None, Some(&query), None)?;
        Ok(response.json()?)
    }

    /// Sends an authenticated `POST /api/{cmd}` request (API v2/v3).
    pub fn authorized_request(
        &self,
        cmd: &str,
        params: Option<Map<String, Value>>,
        version: Option<u8>,
    ) -> Result<Value, TradernetError> {
        let public = self.public.as_ref().ok_or(TradernetError::MissingKeypair)?;
        let private = self.private.as_ref().ok_or(TradernetError::MissingKeypair)?;

        let version = version.unwrap_or(2);
        let params = params.unwrap_or_default();
        let payload = stringify(&Value::Object(params))?;
        let timestamp = current_timestamp();

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if version == 2 || version == 3 {
            let message = format!("{payload}{timestamp}");
            if let Ok(value) = HeaderValue::from_str(public) {
                headers.insert("X-NtApi-PublicKey", value);
            }
            if let Ok(value) = HeaderValue::from_str(&timestamp) {
                headers.insert("X-NtApi-Timestamp", value);
            }
            if let Ok(value) = HeaderValue::from_str(&sign(private, &message)) {
                headers.insert("X-NtApi-Sig", value);
            }
        } else {
            return Err(TradernetError::UnsupportedApiVersion(version));
        }

        let url = format!("{}/api/{}", Self::url(), cmd);
        debug!("Sending POST to {url}");

        let response = self.net.request(Method::POST, &url, Some(headers), None, Some(payload))?;
        let result: Value = response.json()?;

        if result.get("errMsg").is_some() {
            warn!("Error: {:?}", result.get("errMsg"));
        }

        Ok(result)
    }

    /// Sends an authenticated GET request to an API path (API v2/v3).
    pub fn authorized_get_request(
        &self,
        path: &str,
        params: Option<&[(String, String)]>,
        version: Option<u8>,
    ) -> Result<reqwest::blocking::Response, TradernetError> {
        let public = self.public.as_ref().ok_or(TradernetError::MissingKeypair)?;
        let private = self.private.as_ref().ok_or(TradernetError::MissingKeypair)?;

        let version = version.unwrap_or(2);
        if version != 2 && version != 3 {
            return Err(TradernetError::UnsupportedApiVersion(version));
        }

        let timestamp = current_timestamp();
        let message = timestamp.clone();

        let mut headers = HeaderMap::new();
        if let Ok(value) = HeaderValue::from_str(public) {
            headers.insert("X-NtApi-PublicKey", value);
        }
        if let Ok(value) = HeaderValue::from_str(&timestamp) {
            headers.insert("X-NtApi-Timestamp", value);
        }
        if let Ok(value) = HeaderValue::from_str(&sign(private, &message)) {
            headers.insert("X-NtApi-Sig", value);
        }

        let url = format!("{}{}", Self::url(), path);
        debug!("Sending GET to {url}");
        self.net.request(Method::GET, &url, Some(headers), params, None)
    }

    /// Sends an unauthenticated GET request to an API path.
    pub fn get_request(
        &self,
        path: &str,
        params: Option<&[(String, String)]>,
    ) -> Result<reqwest::blocking::Response, TradernetError> {
        let url = format!("{}{}", Self::url(), path);
        debug!("Sending GET to {url}");
        self.net.request(Method::GET, &url, None, params, None)
    }

    /// Returns trading sessions for available securities.
    pub fn list_security_sessions(&self) -> Result<Value, TradernetError> {
        self.authorized_request("getSecuritySessions", None, Some(2))
    }
}

fn current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    now.to_string()
}

fn load_auth_from_ini(path: &Path) -> Result<(Option<String>, Option<String>), TradernetError> {
    let content = fs::read_to_string(path)?;
    let mut in_auth = false;
    let mut public = None;
    let mut private = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            in_auth = &line[1..line.len() - 1] == "auth";
            continue;
        }

        if !in_auth {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "public" => public = Some(value.to_string()),
                "private" => private = Some(value.to_string()),
                _ => {}
            }
        }
    }

    Ok((public, private))
}
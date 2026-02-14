use crate::errors::TradernetError;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::Method;
use std::time::Duration;

/// Blocking HTTP client wrapper used by the SDK.
pub struct NetUtils {
    client: Client,
    /// Request timeout configured for the underlying client.
    pub timeout: Duration,
}

impl NetUtils {
    /// Creates a new HTTP client with the specified timeout.
    #[allow(clippy::result_large_err)]
    pub fn new(timeout: Duration) -> Result<Self, TradernetError> {
        let client = Client::builder().timeout(timeout).build()?;
        Ok(Self { client, timeout })
    }

    /// Sends an HTTP request and returns the response with error status checked.
    #[allow(clippy::result_large_err)]
    pub fn request(
        &self,
        method: Method,
        url: &str,
        headers: Option<HeaderMap>,
        params: Option<&[(String, String)]>,
        body: Option<String>,
    ) -> Result<reqwest::blocking::Response, TradernetError> {
        let mut request = self.client.request(method, url);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        if let Some(params) = params {
            request = request.query(params);
        }

        if let Some(body) = body {
            request = request.body(body);
        }

        let response = request.send()?.error_for_status()?;
        Ok(response)
    }
}
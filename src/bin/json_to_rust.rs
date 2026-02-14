use serde_json::{Map, Value};
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        eprintln!("Не удалось прочитать stdin");
        std::process::exit(1);
    }

    if input.trim().is_empty() {
        eprintln!("Пустой ввод JSON");
        std::process::exit(1);
    }

    let value: Value = match serde_json::from_str(&input) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Некорректный JSON: {err}");
            std::process::exit(1);
        }
    };

    let output = generate_rust(&value);
    println!("{output}");
}

fn generate_rust(value: &Value) -> String {
    let mut context = Context::new();
    let root = context.infer_root(value);
    context.render(root)
}

#[derive(Clone, Debug, PartialEq)]
enum RustType {
    Bool,
    String,
    I64,
    U64,
    F64,
    Vec(Box<RustType>),
    Option(Box<RustType>),
    Struct(String),
    Value,
}

impl RustType {
    fn render(&self) -> String {
        match self {
            RustType::Bool => "bool".to_string(),
            RustType::String => "String".to_string(),
            RustType::I64 => "i64".to_string(),
            RustType::U64 => "u64".to_string(),
            RustType::F64 => "f64".to_string(),
            RustType::Vec(inner) => format!("Vec<{}>", inner.render()),
            RustType::Option(inner) => format!("Option<{}>", inner.render()),
            RustType::Struct(name) => name.clone(),
            RustType::Value => "serde_json::Value".to_string(),
        }
    }
}

struct FieldDef {
    name: String,
    rust_type: RustType,
    serde_rename: Option<String>,
}

struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
}

impl StructDef {
    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("#[derive(Debug, Serialize, Deserialize)]\n");
        out.push_str(&format!("pub struct {} {{\n", self.name));
        for field in &self.fields {
            if let Some(rename) = &field.serde_rename {
                out.push_str(&format!("    #[serde(rename = \"{}\")]\n", rename));
            }
            out.push_str(&format!("    pub {}: {},\n", field.name, field.rust_type.render()));
        }
        out.push_str("}\n");
        out
    }
}

enum RootDef {
    Struct,
    Alias(RustType),
}

struct Context {
    defs: Vec<StructDef>,
    used_names: HashSet<String>,
}

impl Context {
    fn new() -> Self {
        Self {
            defs: Vec::new(),
            used_names: HashSet::new(),
        }
    }

    fn infer_root(&mut self, value: &Value) -> RootDef {
        match value {
            Value::Object(map) => {
                let _ = self.create_struct("Root", map);
                RootDef::Struct
            }
            _ => RootDef::Alias(self.infer_type(value, "Root")),
        }
    }

    fn create_struct(&mut self, base_name: &str, map: &Map<String, Value>) -> String {
        let name = self.unique_struct_name(base_name);
        let mut keys: Vec<&String> = map.keys().collect();
        keys.sort();

        let mut fields = Vec::new();
        for key in keys {
            let value = map.get(key).expect("key exists");
            let (field_name, renamed) = field_name_from_json(key);
            let rust_type = self.infer_type(value, key);
            let serde_rename = if renamed { Some(key.clone()) } else { None };
            fields.push(FieldDef {
                name: field_name,
                rust_type,
                serde_rename,
            });
        }

        self.defs.push(StructDef { name: name.clone(), fields });
        name
    }

    fn infer_type(&mut self, value: &Value, hint: &str) -> RustType {
        match value {
            Value::Null => RustType::Option(Box::new(RustType::Value)),
            Value::Bool(_) => RustType::Bool,
            Value::Number(number) => {
                if number.is_f64() {
                    RustType::F64
                } else if number.is_i64() {
                    RustType::I64
                } else {
                    RustType::U64
                }
            }
            Value::String(_) => RustType::String,
            Value::Array(values) => self.infer_array_type(values, hint),
            Value::Object(map) => {
                let name = self.create_struct(hint, map);
                RustType::Struct(name)
            }
        }
    }

    fn infer_array_type(&mut self, values: &[Value], hint: &str) -> RustType {
        let mut element_type: Option<RustType> = None;
        let mut has_null = false;

        for value in values {
            if value.is_null() {
                has_null = true;
                continue;
            }

            let current = if let Value::Object(map) = value {
                if element_type.is_none() {
                    let struct_name = self.create_struct(&format!("{hint}_item"), map);
                    RustType::Struct(struct_name)
                } else if matches!(element_type, Some(RustType::Struct(_))) {
                    element_type.clone().expect("checked")
                } else {
                    self.infer_type(value, hint)
                }
            } else {
                self.infer_type(value, hint)
            };

            element_type = Some(match element_type {
                None => current,
                Some(existing) => merge_types(existing, current),
            });
        }

        let mut final_type = element_type.unwrap_or(RustType::Value);
        if has_null {
            final_type = RustType::Option(Box::new(final_type));
        }
        RustType::Vec(Box::new(final_type))
    }

    fn unique_struct_name(&mut self, base_name: &str) -> String {
        let base = struct_name_from_json(base_name);
        if self.used_names.insert(base.clone()) {
            return base;
        }

        let mut index = 2;
        loop {
            let candidate = format!("{base}{index}");
            if self.used_names.insert(candidate.clone()) {
                return candidate;
            }
            index += 1;
        }
    }

    fn render(&self, root: RootDef) -> String {
        let mut out = String::new();
        if !self.defs.is_empty() {
            out.push_str("use serde::{Deserialize, Serialize};\n\n");
        }

        for def in &self.defs {
            out.push_str(&def.render());
            out.push('\n');
        }

        if let RootDef::Alias(alias) = root {
            out.push_str(&format!("pub type Root = {};\n", alias.render()));
        }

        out
    }
}

fn merge_types(left: RustType, right: RustType) -> RustType {
    if left == right {
        return left;
    }

    match (left, right) {
        (RustType::Option(inner), other) | (other, RustType::Option(inner)) => {
            if *inner == other {
                RustType::Option(inner)
            } else {
                RustType::Value
            }
        }
        (RustType::Vec(left), RustType::Vec(right)) => {
            RustType::Vec(Box::new(merge_types(*left, *right)))
        }
        _ => RustType::Value,
    }
}

fn field_name_from_json(original: &str) -> (String, bool) {
    let mut name = String::new();
    let mut prev_lower = false;
    let mut changed = false;

    for ch in original.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() {
                if prev_lower && !name.ends_with('_') {
                    name.push('_');
                }
                name.push(ch.to_ascii_lowercase());
                prev_lower = false;
                if ch.is_ascii_uppercase() {
                    changed = true;
                }
            } else {
                name.push(ch.to_ascii_lowercase());
                prev_lower = ch.is_ascii_lowercase() || ch.is_ascii_digit();
            }
        } else {
            if !name.ends_with('_') {
                name.push('_');
            }
            prev_lower = false;
            changed = true;
        }
    }

    while name.starts_with('_') {
        name.remove(0);
        changed = true;
    }
    while name.ends_with('_') {
        name.pop();
        changed = true;
    }

    while name.contains("__") {
        name = name.replace("__", "_");
        changed = true;
    }

    if name.is_empty() {
        name = "field".to_string();
        changed = true;
    }

    if name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        name = format!("_{name}");
        changed = true;
    }

    if is_rust_keyword(&name) {
        name.push('_');
        changed = true;
    }

    if name != original {
        changed = true;
    }

    (name, changed)
}

fn struct_name_from_json(original: &str) -> String {
    let mut name = String::new();
    let mut next_upper = true;
    for ch in original.chars() {
        if ch.is_ascii_alphanumeric() {
            if next_upper {
                name.push(ch.to_ascii_uppercase());
                next_upper = false;
            } else {
                name.push(ch.to_ascii_lowercase());
            }
        } else {
            next_upper = true;
        }
    }

    if name.is_empty() {
        name.push_str("Generated");
    }

    if name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        name.insert(0, 'N');
    }

    if is_rust_keyword(&name) {
        name.push_str("Type");
    }

    name
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as"
            | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

#[cfg(test)]
mod tests {
    use super::generate_rust;
    use serde_json::Value;

    #[test]
    fn generates_basic_struct() {
        let value: Value = serde_json::from_str(
            r#"{"id":1,"price":1.5,"active":true,"name":"Test"}"#,
        )
        .unwrap();

        let output = generate_rust(&value);
        let expected = "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Serialize, Deserialize)]\npub struct Root {\n    pub active: bool,\n    pub id: i64,\n    pub name: String,\n    pub price: f64,\n}\n\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn generates_nested_and_option_fields() {
        let value: Value = serde_json::from_str(
            r#"{"user":{"id":1,"email":null},"tags":["a","b"],"items":[{"code":"A","qty":2}]}"#,
        )
        .unwrap();

        let output = generate_rust(&value);
        let expected = "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Serialize, Deserialize)]\npub struct ItemsItem {\n    pub code: String,\n    pub qty: i64,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\npub struct User {\n    pub email: Option<serde_json::Value>,\n    pub id: i64,\n}\n\n#[derive(Debug, Serialize, Deserialize)]\npub struct Root {\n    pub items: Vec<ItemsItem>,\n    pub tags: Vec<String>,\n    pub user: User,\n}\n\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn generates_root_alias_for_array() {
        let value: Value = serde_json::from_str(r#"[{"id":1},{"id":2}]"#).unwrap();

        let output = generate_rust(&value);
        let expected = "use serde::{Deserialize, Serialize};\n\n#[derive(Debug, Serialize, Deserialize)]\npub struct RootItem {\n    pub id: i64,\n}\n\npub type Root = Vec<RootItem>;\n";

        assert_eq!(output, expected);
    }
}
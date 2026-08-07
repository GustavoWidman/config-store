use eyre::{Result, eyre};

// Use TOML if the toml feature is enabled
#[cfg(feature = "toml")]
pub use toml::{Value, from_str, to_string};
// Use JSON if the json feature is enabled and toml is not
#[cfg(all(feature = "json", not(feature = "toml")))]
pub use serde_json::{Value, from_str, to_string};
// Use YAML if the yaml feature is enabled and neither toml nor json are
#[cfg(all(feature = "yaml", not(feature = "toml"), not(feature = "json")))]
pub use serde_yaml::{Value, from_str, to_string};

pub fn merge_values(config: &mut Value, other: Value) -> Result<()> {
    merge_values_at(config, other, "config")
}

pub fn overwrite_values(config: &mut Value, other: Value) {
    overwrite_values_at(config, other)
}

#[cfg(feature = "toml")]
fn merge_values_at(config: &mut Value, other: Value, path: &str) -> Result<()> {
    match (config, other) {
        (Value::Table(config), Value::Table(other)) => {
            for (key, other_value) in other {
                let path = format!("{path}.{key}");
                match config.get_mut(&key) {
                    Some(config_value) => merge_values_at(config_value, other_value, &path)?,
                    None => {
                        config.insert(key, other_value);
                    }
                }
            }
            Ok(())
        }
        (config, other) if *config == other => Ok(()),
        (config, other) => Err(eyre!(
            "conflicting config value at {path}: existing {} differs from incoming {}",
            value_kind(config),
            value_kind(&other)
        )),
    }
}

#[cfg(feature = "toml")]
fn overwrite_values_at(config: &mut Value, other: Value) {
    match (config, other) {
        (Value::Table(config), Value::Table(other)) => {
            for (key, other_value) in other {
                match config.get_mut(&key) {
                    Some(config_value) => overwrite_values_at(config_value, other_value),
                    None => {
                        config.insert(key, other_value);
                    }
                }
            }
        }
        (config, other) => *config = other,
    }
}

#[cfg(feature = "toml")]
fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "string",
        Value::Integer(_) => "integer",
        Value::Float(_) => "float",
        Value::Boolean(_) => "boolean",
        Value::Datetime(_) => "datetime",
        Value::Array(_) => "array",
        Value::Table(_) => "table",
    }
}

#[cfg(all(feature = "json", not(feature = "toml")))]
fn merge_values_at(config: &mut Value, other: Value, path: &str) -> Result<()> {
    match (config, other) {
        (Value::Object(config), Value::Object(other)) => {
            for (key, other_value) in other {
                let path = format!("{path}.{key}");
                match config.get_mut(&key) {
                    Some(config_value) => merge_values_at(config_value, other_value, &path)?,
                    None => {
                        config.insert(key, other_value);
                    }
                }
            }
            Ok(())
        }
        (config, other) if *config == other => Ok(()),
        (config, other) => Err(eyre!(
            "conflicting config value at {path}: existing {} differs from incoming {}",
            value_kind(config),
            value_kind(&other)
        )),
    }
}

#[cfg(all(feature = "json", not(feature = "toml")))]
fn overwrite_values_at(config: &mut Value, other: Value) {
    match (config, other) {
        (Value::Object(config), Value::Object(other)) => {
            for (key, other_value) in other {
                match config.get_mut(&key) {
                    Some(config_value) => overwrite_values_at(config_value, other_value),
                    None => {
                        config.insert(key, other_value);
                    }
                }
            }
        }
        (config, other) => *config = other,
    }
}

#[cfg(all(feature = "json", not(feature = "toml")))]
fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(all(feature = "yaml", not(feature = "toml"), not(feature = "json")))]
fn merge_values_at(config: &mut Value, other: Value, path: &str) -> Result<()> {
    match (config, other) {
        (Value::Mapping(config), Value::Mapping(other)) => {
            for (key, other_value) in other {
                let key_label = key.as_str().unwrap_or("<non-string-key>");
                let path = format!("{path}.{key_label}");
                match config.get_mut(&key) {
                    Some(config_value) => merge_values_at(config_value, other_value, &path)?,
                    None => {
                        config.insert(key, other_value);
                    }
                }
            }
            Ok(())
        }
        (config, other) if *config == other => Ok(()),
        (config, other) => Err(eyre!(
            "conflicting config value at {path}: existing {} differs from incoming {}",
            value_kind(config),
            value_kind(&other)
        )),
    }
}

#[cfg(all(feature = "yaml", not(feature = "toml"), not(feature = "json")))]
fn overwrite_values_at(config: &mut Value, other: Value) {
    match (config, other) {
        (Value::Mapping(config), Value::Mapping(other)) => {
            for (key, other_value) in other {
                match config.get_mut(&key) {
                    Some(config_value) => overwrite_values_at(config_value, other_value),
                    None => {
                        config.insert(key, other_value);
                    }
                }
            }
        }
        (config, other) => *config = other,
    }
}

#[cfg(all(feature = "yaml", not(feature = "toml"), not(feature = "json")))]
fn value_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Sequence(_) => "sequence",
        Value::Mapping(_) => "mapping",
        Value::Tagged(_) => "tagged",
    }
}

#[cfg(not(any(feature = "toml", feature = "json", feature = "yaml",)))]
pub fn from_str<'de, T>(s: &'de str) -> Result<T>
where
    T: Deserialize<'de>,
{
    compile_error!(
        "No serialization features are enabled. Enable one of the following features: toml, json, yaml"
    );
}

#[cfg(not(any(feature = "toml", feature = "json", feature = "yaml",)))]
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: ?Sized + ser::Serialize,
{
    compile_error!(
        "No serialization features are enabled. Enable one of the following features: toml, json, yaml"
    );
}

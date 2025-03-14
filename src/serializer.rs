// Use TOML if the toml feature is enabled
#[cfg(feature = "toml")]
pub use toml::{from_str, to_string};
// Use JSON if the json feature is enabled and toml is not
#[cfg(all(feature = "json", not(feature = "toml")))]
pub use serde_json::{from_str, to_string};
// Use YAML if the yaml feature is enabled and neither toml nor json are
#[cfg(all(feature = "yaml", not(feature = "toml"), not(feature = "json")))]
pub use serde_yaml::{from_str, to_string};

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

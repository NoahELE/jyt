//! Provide a general way of converting different format,
//! currently include json, yaml and toml. (powered by serde)
pub mod error;
pub mod json;
pub mod toml;
pub mod yaml;

pub use ::toml::Value as TomlValue;
pub use serde_json::Value as JsonValue;
pub use serde_yaml::Value as YamlValue;

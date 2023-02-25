//! Provide a general way of converting different format,
//! currently include json, yaml and toml. (powered by serde)
pub(crate) mod error;
pub(crate) mod json;
pub(crate) mod toml;
pub(crate) mod yaml;

pub(crate) use ::toml::Value as TomlValue;
pub(crate) use serde_json::Value as JsonValue;
pub(crate) use serde_yaml::Value as YamlValue;

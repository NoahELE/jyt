//! Errors that might occur during serialization and deserialization.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerError {
    #[error("JSON serialize error {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML serialize error {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("TOML serialize error {0}")]
    Toml(#[from] toml::ser::Error),
}

#[derive(Error, Debug)]
pub enum DeError {
    #[error("JSON deserialize error {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML deserialize error {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("TOML deserialize error {0}")]
    Toml(#[from] toml::de::Error),
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JSON serialize error ({0})")]
    JsonSer(serde_json::Error),
    #[error("JSON deserialize error ({0})")]
    JsonDe(serde_json::Error),
    #[error("YAML serialize error ({0})")]
    YamlSer(serde_yaml::Error),
    #[error("YAML deserialize error ({0})")]
    YamlDe(serde_yaml::Error),
    #[error("TOML serialize error ({0})")]
    TomlSer(toml::ser::Error),
    #[error("TOML deserialize error ({0})")]
    TomlDe(toml::de::Error),
}

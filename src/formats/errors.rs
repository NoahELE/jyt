use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JSON serialize error ({0})")]
    JsonSer(String),
    #[error("JSON deserialize error ({0})")]
    JsonDe(String),
    #[error("YAML serialize error ({0})")]
    YamlSer(String),
    #[error("YAML deserialize error ({0})")]
    YamlDe(String),
    #[error("TOML serialize error ({0})")]
    TomlSer(String),
    #[error("TOML deserialize error ({0})")]
    TomlDe(String),
}

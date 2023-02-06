//! `toml` related serialization and deserialization
use super::error::Error;

pub fn serialize<V: serde::Serialize>(v: V) -> Result<String, Error> {
    toml::to_string_pretty(&v).map_err(Error::TomlSer)
}

pub fn deserialize<V>(s: &str) -> Result<V, Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    toml::from_str(s).map_err(Error::TomlDe)
}

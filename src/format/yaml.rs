//! `yaml` related serialization and deserialization
use super::error::Error;

pub fn serialize<V: serde::Serialize>(v: V) -> Result<String, Error> {
    serde_yaml::to_string(&v).map_err(Error::YamlSer)
}

pub fn deserialize<V>(s: &str) -> Result<V, Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    serde_yaml::from_str(s).map_err(Error::YamlDe)
}

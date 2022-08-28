use super::errors::Error;

pub fn serialize<V: serde::Serialize>(v: V) -> Result<String, Error> {
    toml::to_string(&v).map_err(|e| Error::TomlSer(e.to_string()))
}

pub fn deserialize<V>(s: &str) -> Result<V, Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    toml::from_str(s).map_err(|e| Error::TomlDe(e.to_string()))
}

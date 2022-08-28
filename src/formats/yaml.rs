use super::errors::Error;

pub fn serialize<V: serde::Serialize>(v: V) -> Result<String, Error> {
    serde_yaml::to_string(&v).map_err(|e| Error::YamlSer(e.to_string()))
}

pub fn deserialize<V>(s: &str) -> Result<V, Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    serde_yaml::from_str(s).map_err(|e| Error::YamlDe(e.to_string()))
}

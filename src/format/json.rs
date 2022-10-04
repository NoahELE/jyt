use super::error::Error;

pub fn serialize<V: serde::Serialize>(v: V) -> Result<String, Error> {
    serde_json::to_string_pretty(&v).map_err(Error::JsonSer)
}

pub fn deserialize<V>(s: &str) -> Result<V, Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    serde_json::from_str(s).map_err(Error::JsonDe)
}

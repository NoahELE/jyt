use super::errors::Error;

pub fn serialize<V: serde::Serialize>(v: V) -> Result<String, Error> {
    serde_json::to_string_pretty(&v).map_err(|e| Error::JsonSer(e.to_string()))
}

pub fn deserialize<V>(s: &str) -> Result<V, Error>
where
    V: for<'de> serde::Deserialize<'de>,
{
    serde_json::from_str(s).map_err(|e| Error::JsonDe(e.to_string()))
}

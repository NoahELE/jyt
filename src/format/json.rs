//! `json` related serialization and deserialization
use serde::{Deserialize, Serialize};

use super::error::{DeError, SerError};

pub fn serialize<V: Serialize>(v: V) -> Result<String, SerError> {
    serde_json::to_string_pretty(&v).map_err(SerError::from)
}

pub fn deserialize<V>(s: &str) -> Result<V, DeError>
where
    V: for<'de> Deserialize<'de>,
{
    serde_json::from_str(s).map_err(DeError::from)
}

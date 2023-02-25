//! `toml` related serialization and deserialization
use serde::{Deserialize, Serialize};

use super::error::{DeError, SerError};

pub(crate) fn serialize<V: Serialize>(v: V) -> Result<String, SerError> {
    toml::to_string_pretty(&v).map_err(Into::into)
}

pub(crate) fn deserialize<V>(s: &str) -> Result<V, DeError>
where
    V: for<'de> Deserialize<'de>,
{
    toml::from_str(s).map_err(Into::into)
}

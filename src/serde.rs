use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

use crate::Snowflake;

impl Serialize for Snowflake {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer {
    serializer.serialize_str(&self.value().to_string())
  }
}

impl<'de> Deserialize<'de> for Snowflake {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where D: serde::Deserializer<'de> {
    let value: u64 = deserialize_number_from_string(deserializer)?;

    Ok(Self::from_value(value))
  }
}

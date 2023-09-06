use serde::Serialize;

use crate::Snowflake;

impl Serialize for Snowflake {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer {
    serializer.serialize_str(&self.value().to_string())
  }
}

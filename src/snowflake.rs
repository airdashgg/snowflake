use std::fmt::Display;
use std::str::FromStr;

use chrono::{DateTime, Utc};

/// ```md
/// |                                            worker
/// |                                            │     process
/// | timestamp                                  │     │     increment
/// | │                                          │     │     │
/// | 111111111111111111111111111111111111111111 11111 11111 111111111111
/// | 63                                        22    17    12          0
///
/// Max values:
/// worker: 31
/// process: 31
/// increment: 4095
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(specta::Type))]
#[cfg_attr(feature = "serde", specta(transparent))]
pub struct Snowflake(#[specta(type = String)] u64);

#[inline]
const fn value_from_parts(timestamp: u64, worker: u8, process: u8, increment: u16) -> u64 {
  let mut value = timestamp << 22;
  value |= (worker as u64) << 17;
  value |= (process as u64) << 12;
  value |= increment as u64;

  value
}

impl Snowflake {
  #[inline]
  pub fn new(worker: u8, process: u8, increment: u16, epoch: u64) -> Self {
    Self::new_with_timestamp(worker, process, increment, Utc::now(), epoch)
  }

  #[inline]
  pub const fn new_with_timestamp(
    worker: u8,
    process: u8,
    increment: u16,
    timestamp: DateTime<Utc>,
    epoch: u64,
  ) -> Self {
    let offset_timestamp_ms = timestamp.timestamp_millis() - epoch as i64;

    let value = value_from_parts(offset_timestamp_ms as u64, worker, process, increment);

    Self(value)
  }

  #[inline]
  pub const fn from_value(value: u64) -> Self { Self(value) }

  #[inline]
  pub const fn as_i64(&self) -> i64 { self.value() as i64 }

  #[inline]
  pub const fn as_u64(&self) -> u64 { self.value() }

  #[inline]
  pub const fn value(&self) -> u64 { self.0 }

  #[inline]
  pub const fn worker(&self) -> u8 { ((self.value() >> 17) & 0b11111) as u8 }

  #[inline]
  pub const fn process(&self) -> u8 { ((self.value() >> 12) & 0b11111) as u8 }

  #[inline]
  pub const fn increment(&self) -> u16 { (self.value() & 0b111111111111) as u16 }

  #[inline]
  pub const fn timestamp_raw(&self) -> u64 { self.value() >> 22 }

  #[inline]
  pub const fn timestamp(&self, epoch: u64) -> u64 { self.timestamp_raw() + epoch }
}

impl FromStr for Snowflake {
  type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let value = u64::from_str(s)?;
    Ok(Self::from_value(value))
  }
}

impl Display for Snowflake {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value()) }
}

impl std::fmt::Debug for Snowflake {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      f.debug_struct("Snowflake")
        .field("worker", &self.worker())
        .field("process", &self.process())
        .field("increment", &self.increment())
        .field("timestamp", &self.timestamp_raw())
        .finish()
    } else {
      write!(f, "{self}")
    }
  }
}

impl From<u64> for Snowflake {
  fn from(value: u64) -> Self { Self::from_value(value) }
}

impl From<i64> for Snowflake {
  fn from(value: i64) -> Self { Self::from_value(value as u64) }
}

#[cfg(test)]
mod tests {
  use chrono::TimeZone;

  use super::*;
  use crate::AIRDASH_EPOCH;

  const WORKER: u8 = 8;
  const PROCESS: u8 = 26;
  const INCREMENT: u16 = 543;

  #[test]
  fn test_new() {
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT, AIRDASH_EPOCH);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_ne!(snowflake.timestamp_raw(), 0);
  }

  #[test]
  fn test_new_with_timestamp() {
    let timestamp = Utc.with_ymd_and_hms(2022, 7, 8, 9, 10, 11).unwrap();

    let snowflake = Snowflake::new_with_timestamp(WORKER, PROCESS, INCREMENT, timestamp, AIRDASH_EPOCH);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(snowflake.timestamp(AIRDASH_EPOCH), timestamp.timestamp_millis() as u64);
  }

  #[test]
  fn test_new_with_epoch() {
    let epoch_timestamp = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap().timestamp_millis() as u64;

    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT, epoch_timestamp);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(
      snowflake.timestamp(AIRDASH_EPOCH),
      snowflake.timestamp_raw() + epoch_timestamp
    );
  }

  #[test]
  fn test_new_with_timestamp_and_epoch() {
    let timestamp = Utc.with_ymd_and_hms(2022, 7, 8, 9, 10, 11).unwrap();
    let epoch_timestamp = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap().timestamp_millis() as u64;

    let snowflake = Snowflake::new_with_timestamp(WORKER, PROCESS, INCREMENT, timestamp, epoch_timestamp);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(
      snowflake.timestamp(epoch_timestamp),
      timestamp.timestamp_millis() as u64
    );
    assert_eq!(
      snowflake.timestamp_raw(),
      timestamp.timestamp_millis() as u64 - epoch_timestamp
    );
  }

  #[test]
  fn test_from_value() {
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT, AIRDASH_EPOCH);

    let value = snowflake.value();

    let from_value = Snowflake::from_value(value);

    assert_eq!(from_value, snowflake);
  }

  #[test]
  fn test_from_value_with_epoch() {
    let epoch_timestamp = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap().timestamp_millis() as u64;
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT, epoch_timestamp);

    let value = snowflake.value();

    let from_value = Snowflake::from_value(value);

    assert_eq!(from_value, snowflake);
  }
}

use std::fmt::Display;

use proc_bitfield::bitfield;
#[cfg(feature = "serde")]
use serde::Serialize;
use time::OffsetDateTime;

use crate::{millis, AIRDASH_EPOCH};

bitfield! {
  /// ```md
  ///                                            worker
  ///                                            │     process
  /// timestamp                                  │     │     increment
  /// │                                          │     │     │
  /// 111111111111111111111111111111111111111111 11111 11111 111111111111
  /// 63                                        22    17    12          0
  ///
  /// Max values:
  /// worker: 31
  /// process: 31
  /// increment: 4095
  /// ```
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub struct Snowflake(pub u128): Debug {
    pub increment: u16 @ 0..12,
    pub process: u8 @ 12..17,
    pub worker: u8 @ 17..22,
    pub timestamp: u64 @ 22..64,
    pub epoch: u64 @ 64..128,
    pub value: u64 [read_only] @ 0..64,
  }
}

impl Snowflake {
  pub fn new(worker: u8, process: u8, increment: u16) -> Self {
    Self::new_with_timestamp_and_epoch(worker, process, increment, OffsetDateTime::now_utc(), AIRDASH_EPOCH)
  }

  pub fn new_with_timestamp(worker: u8, process: u8, increment: u16, timestamp: OffsetDateTime) -> Self {
    Self::new_with_timestamp_and_epoch(worker, process, increment, timestamp, AIRDASH_EPOCH)
  }

  pub fn new_with_epoch(worker: u8, process: u8, increment: u16, epoch: u64) -> Self {
    Self::new_with_timestamp_and_epoch(worker, process, increment, OffsetDateTime::now_utc(), epoch)
  }

  pub fn new_with_timestamp_and_epoch(
    worker: u8,
    process: u8,
    increment: u16,
    timestamp: OffsetDateTime,
    epoch: u64,
  ) -> Self {
    let offset_timestamp_ms = millis(timestamp) - epoch;

    Self(0)
      .with_worker(worker)
      .with_process(process)
      .with_increment(increment)
      .with_timestamp(offset_timestamp_ms)
      .with_epoch(epoch)
  }

  pub fn from_value(value: u64) -> Self { Self(value as u128).with_epoch(AIRDASH_EPOCH) }

  pub fn from_value_with_epoch(value: u64, epoch: u64) -> Self { Self(value as u128).with_epoch(epoch) }

  pub fn offset_timestamp(&self) -> u64 { self.timestamp() + self.epoch() }
}

impl Display for Snowflake {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value()) }
}

#[cfg(feature = "serde")]
impl Serialize for Snowflake {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer {
    serializer.serialize_str(&self.value().to_string())
  }
}

#[cfg(test)]
mod tests {
  use time::macros::datetime;

  use super::*;

  const WORKER: u8 = 8;
  const PROCESS: u8 = 26;
  const INCREMENT: u16 = 543;

  #[test]
  fn test_new() {
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_ne!(snowflake.timestamp(), 0);
  }

  #[test]
  fn test_new_with_timestamp() {
    let timestamp = datetime!(2022-07-08 09:10:11).assume_utc();

    let snowflake = Snowflake::new_with_timestamp(WORKER, PROCESS, INCREMENT, timestamp);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(snowflake.offset_timestamp(), millis(timestamp));
  }

  #[test]
  fn test_new_with_epoch() {
    let epoch_timestamp = datetime!(2014-07-08 09:10:11).assume_utc();

    let snowflake = Snowflake::new_with_epoch(WORKER, PROCESS, INCREMENT, millis(epoch_timestamp));

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(
      snowflake.offset_timestamp(),
      snowflake.timestamp() + millis(epoch_timestamp)
    );
  }

  #[test]
  fn test_new_with_timestamp_and_epoch() {
    let timestamp = datetime!(2022-07-08 09:10:11).assume_utc();
    let epoch_timestamp = datetime!(2014-07-08 09:10:11).assume_utc();

    let snowflake =
      Snowflake::new_with_timestamp_and_epoch(WORKER, PROCESS, INCREMENT, timestamp, millis(epoch_timestamp));

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(snowflake.offset_timestamp(), millis(timestamp));
    assert_eq!(
      snowflake.offset_timestamp(),
      snowflake.timestamp() + millis(epoch_timestamp)
    );
  }

  #[test]
  fn test_from_value() {
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT);

    let value = snowflake.value();

    let from_value = Snowflake::from_value(value);

    assert_eq!(from_value, snowflake);
  }

  #[test]
  fn test_from_value_with_epoch() {
    let epoch_timestamp = datetime!(2014-07-08 09:10:11).assume_utc();
    let snowflake = Snowflake::new_with_epoch(WORKER, PROCESS, INCREMENT, millis(epoch_timestamp));

    let value = snowflake.value();

    let from_value = Snowflake::from_value_with_epoch(value, millis(epoch_timestamp));

    assert_eq!(from_value, snowflake);
  }
}

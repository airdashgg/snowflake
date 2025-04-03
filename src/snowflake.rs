use std::fmt::Display;

use chrono::{DateTime, Utc};
use proc_bitfield::bitfield;

use crate::AIRDASH_EPOCH;

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
  #[cfg_attr(feature = "serde", derive(specta::Type))]
  pub struct Snowflake(pub u128) {
    pub increment: u16 @ 0..12,
    pub process: u8 @ 12..17,
    pub worker: u8 @ 17..22,
    pub timestamp: u64 @ 22..64,
    pub epoch: u64 @ 64..128,
    pub value: u64 [read_only] @ 0..64,
  }
}

impl Snowflake {
  pub fn new(worker: u8, process: u8, increment: u16, epoch: u64) -> Self {
    Self::new_with_timestamp(worker, process, increment, Utc::now(), epoch)
  }

  pub fn new_with_timestamp(worker: u8, process: u8, increment: u16, timestamp: DateTime<Utc>, epoch: u64) -> Self {
    let offset_timestamp_ms = timestamp.timestamp_millis() - epoch as i64;

    Self(0)
      .with_worker(worker)
      .with_process(process)
      .with_increment(increment)
      .with_timestamp(offset_timestamp_ms as u64)
      .with_epoch(epoch)
  }

  pub fn from_value(value: u64, epoch: u64) -> Self { Self(value as u128).with_epoch(epoch) }

  pub fn new_airdash_epoch(worker: u8, process: u8, increment: u16) -> Self {
    Self::new_with_timestamp(worker, process, increment, Utc::now(), AIRDASH_EPOCH)
  }

  pub fn new_with_timestamp_airdash_epoch(worker: u8, process: u8, increment: u16, timestamp: DateTime<Utc>) -> Self {
    Self::new_with_timestamp(worker, process, increment, timestamp, AIRDASH_EPOCH)
  }

  pub fn from_value_airdash_epoch(value: u64) -> Self { Self(value as u128).with_epoch(AIRDASH_EPOCH) }

  pub fn offset_timestamp(&self) -> u64 { self.timestamp() + self.epoch() }

  pub fn as_i64(&self) -> i64 { self.value() as i64 }

  pub fn as_u64(&self) -> u64 { self.value() }
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
        .field("timestamp", &self.timestamp())
        .field("epoch", &self.epoch())
        .finish()
    } else {
      write!(f, "{self}")
    }
  }
}

impl From<u64> for Snowflake {
  fn from(value: u64) -> Self { Self::from_value_airdash_epoch(value) }
}

impl From<i64> for Snowflake {
  fn from(value: i64) -> Self { (value as u64).into() }
}

#[cfg(test)]
mod tests {
  use super::*;

  const WORKER: u8 = 8;
  const PROCESS: u8 = 26;
  const INCREMENT: u16 = 543;

  #[test]
  fn test_new() {
    let snowflake = Snowflake::new_airdash_epoch(WORKER, PROCESS, INCREMENT);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_ne!(snowflake.timestamp(), 0);
  }

  #[test]
  fn test_new_with_timestamp() {
    let timestamp = datetime!(2022-07-08 09:10:11).assume_utc();

    let snowflake = Snowflake::new_with_timestamp_airdash_epoch(WORKER, PROCESS, INCREMENT, timestamp);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(snowflake.offset_timestamp(), millis(timestamp));
  }

  #[test]
  fn test_new_with_epoch() {
    let epoch_timestamp = datetime!(2014-07-08 09:10:11).assume_utc();

    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT, millis(epoch_timestamp));

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

    let snowflake = Snowflake::new_with_timestamp(WORKER, PROCESS, INCREMENT, timestamp, millis(epoch_timestamp));

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
    let snowflake = Snowflake::new_airdash_epoch(WORKER, PROCESS, INCREMENT);

    let value = snowflake.value();

    let from_value = Snowflake::from_value_airdash_epoch(value);

    assert_eq!(from_value, snowflake);
  }

  #[test]
  fn test_from_value_with_epoch() {
    let epoch_timestamp = datetime!(2014-07-08 09:10:11).assume_utc();
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT, millis(epoch_timestamp));

    let value = snowflake.value();

    let from_value = Snowflake::from_value(value, millis(epoch_timestamp));

    assert_eq!(from_value, snowflake);
  }
}

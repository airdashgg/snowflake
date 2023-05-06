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
    Self::new_with_timestamp_and_epoch(worker, process, increment, Utc::now(), AIRDASH_EPOCH)
  }

  pub fn new_with_timestamp(worker: u8, process: u8, increment: u16, timestamp: DateTime<Utc>) -> Self {
    Self::new_with_timestamp_and_epoch(worker, process, increment, timestamp, AIRDASH_EPOCH)
  }

  pub fn new_with_epoch(worker: u8, process: u8, increment: u16, epoch: u64) -> Self {
    Self::new_with_timestamp_and_epoch(worker, process, increment, Utc::now(), epoch)
  }

  pub fn new_with_timestamp_and_epoch(
    worker: u8,
    process: u8,
    increment: u16,
    timestamp: DateTime<Utc>,
    epoch: u64,
  ) -> Self {
    let offset_timestamp_ms = (timestamp.timestamp_millis() as u64) - epoch;

    Self(0)
      .with_worker(worker)
      .with_process(process)
      .with_increment(increment)
      .with_timestamp(offset_timestamp_ms)
      .with_epoch(epoch)
  }

  pub fn offset_timestamp(&self) -> u64 { self.timestamp() + self.epoch() }
}

impl Display for Snowflake {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.value()) }
}

#[cfg(test)]
mod tests {
  use chrono::TimeZone;

  use super::*;

  const WORKER: u8 = 8;
  const PROCESS: u8 = 26;
  const INCREMENT: u16 = 543;

  #[test]
  fn test_new() {
    let snowflake = Snowflake::new(WORKER, PROCESS, INCREMENT);

    println!("{:b}", snowflake.0);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_ne!(snowflake.timestamp(), 0);
  }

  #[test]
  fn test_new_with_timestamp() {
    let timestamp = Utc.with_ymd_and_hms(2022, 7, 8, 9, 10, 11).unwrap();

    let snowflake = Snowflake::new_with_timestamp(WORKER, PROCESS, INCREMENT, timestamp);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(snowflake.offset_timestamp(), timestamp.timestamp_millis() as u64);
  }

  #[test]
  fn test_new_with_epoch() {
    let epoch_timestamp = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();

    let snowflake = Snowflake::new_with_epoch(WORKER, PROCESS, INCREMENT, epoch_timestamp.timestamp_millis() as u64);

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(
      snowflake.offset_timestamp(),
      snowflake.timestamp() + epoch_timestamp.timestamp_millis() as u64
    );
  }

  #[test]
  fn test_new_with_timestamp_and_epoch() {
    let timestamp = Utc.with_ymd_and_hms(2022, 7, 8, 9, 10, 11).unwrap();
    let epoch_timestamp = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap();

    let snowflake = Snowflake::new_with_timestamp_and_epoch(
      WORKER,
      PROCESS,
      INCREMENT,
      timestamp,
      epoch_timestamp.timestamp_millis() as u64,
    );

    assert_eq!(snowflake.worker(), WORKER);
    assert_eq!(snowflake.process(), PROCESS);
    assert_eq!(snowflake.increment(), INCREMENT);
    assert_eq!(snowflake.offset_timestamp(), timestamp.timestamp_millis() as u64);
    assert_eq!(
      snowflake.offset_timestamp(),
      snowflake.timestamp() + epoch_timestamp.timestamp_millis() as u64
    );
  }
}

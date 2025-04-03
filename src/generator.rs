use crate::{Snowflake, AIRDASH_EPOCH};

#[derive(Debug)]
pub struct SnowflakeGenerator {
  worker: u8,
  process: u8,
  increment: u16,
  epoch: u64,
}

impl SnowflakeGenerator {
  pub fn new(worker: u8, process: u8) -> Self { Self::new_with_epoch(worker, process, AIRDASH_EPOCH) }

  pub fn new_with_epoch(worker: u8, process: u8, epoch: u64) -> Self {
    Self {
      epoch,
      worker,
      process,
      increment: 0,
    }
  }

  pub fn generate(&mut self) -> Snowflake {
    let snowflake = Snowflake::new(self.worker, self.process, self.increment, self.epoch);

    self.increment = self.increment.wrapping_add(1);

    snowflake
  }
}

impl Default for SnowflakeGenerator {
  fn default() -> Self { Self::new(0, 0) }
}

impl Iterator for SnowflakeGenerator {
  type Item = Snowflake;

  fn next(&mut self) -> Option<Self::Item> { Some(self.generate()) }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use time::OffsetDateTime;

  use super::*;
  use crate::millis;

  const WORKER: u8 = 8;
  const PROCESS: u8 = 26;
  const GENERATED_COUNT: usize = 500_000;

  #[test]
  fn test_generates_no_duplicates() {
    let generator = SnowflakeGenerator::new(WORKER, PROCESS);

    let snowflakes = generator.take(GENERATED_COUNT).collect::<Vec<Snowflake>>();

    let mut unique_snowflakes = HashSet::new();

    for snowflake in snowflakes {
      // HashSet::insert() returns `false` if the value already exists
      assert!(unique_snowflakes.insert(snowflake.value()));
    }
  }

  #[test]
  fn test_generates_correct_values() {
    let start_time = OffsetDateTime::now_utc();

    let generator = SnowflakeGenerator::new(WORKER, PROCESS);

    let snowflakes = generator.take(GENERATED_COUNT).collect::<Vec<Snowflake>>();

    assert_eq!(snowflakes.len(), GENERATED_COUNT);

    for snowflake in snowflakes {
      assert_eq!(snowflake.worker(), WORKER);
      assert_eq!(snowflake.process(), PROCESS);

      assert!(snowflake.offset_timestamp() >= millis(start_time));
    }
  }
}

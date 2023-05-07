mod generator;
mod snowflake;

use time::OffsetDateTime;

pub use crate::generator::SnowflakeGenerator;
pub use crate::snowflake::Snowflake;

pub const AIRDASH_EPOCH: u64 = 1420070400000;

#[inline]
pub(crate) const fn millis(datetime: OffsetDateTime) -> u64 { (datetime.unix_timestamp_nanos() / 1_000_000) as u64 }

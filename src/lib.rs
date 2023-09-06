mod generator;
#[cfg(feature = "serde")]
mod serde;
mod snowflake;
#[cfg(feature = "ts_rs")]
mod ts_rs;

use time::OffsetDateTime;

pub use crate::generator::SnowflakeGenerator;
pub use crate::snowflake::Snowflake;

pub const AIRDASH_EPOCH: u64 = 1420070400000;

#[inline]
pub(crate) const fn millis(datetime: OffsetDateTime) -> u64 { (datetime.unix_timestamp_nanos() / 1_000_000) as u64 }

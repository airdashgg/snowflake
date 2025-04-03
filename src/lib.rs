mod generator;
#[cfg(feature = "serde")]
mod serde;
mod snowflake;

pub use crate::generator::SnowflakeGenerator;
pub use crate::snowflake::Snowflake;

pub const AIRDASH_EPOCH: u64 = 1420070400000;

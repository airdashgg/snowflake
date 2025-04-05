mod generator;
#[cfg(feature = "serde")]
mod serde;
mod snowflake;

pub use crate::generator::SnowflakeGenerator;
pub use crate::snowflake::Snowflake;

pub const AIRDASH_EPOCH: u64 = 1420070400000;

pub const WORKER_MAX: u8 = 31;
pub const PROCESS_MAX: u8 = 31;
pub const INCREMENT_MAX: u16 = 4095;
pub const TIMESTAMP_MAX: u64 = 0x1FFFFFFFFFF;

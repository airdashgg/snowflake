mod generator;
mod snowflake;

pub use generator::SnowflakeGenerator;
pub use snowflake::Snowflake;

pub const AIRDASH_EPOCH: u64 = 1420070400000;

use ts_rs::TS;

use crate::Snowflake;

const NAME: &str = "Snowflake";
const TYPE: &str = "string";

impl TS for Snowflake {
  fn decl() -> String { format!("type {NAME} = {TYPE};") }

  fn name() -> String { NAME.into() }

  fn inline() -> String { TYPE.into() }

  fn dependencies() -> Vec<ts_rs::Dependency>
  where Self: 'static {
    vec![]
  }

  fn transparent() -> bool { false }
}

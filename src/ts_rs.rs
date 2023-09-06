use ts_rs::TS;

use crate::Snowflake;

const NAME: &str = "Snowflake";
const TYPE: &str = "string";

impl TS for Snowflake {
  const EXPORT_TO: Option<&'static str> = Some("bindings/Snowflake.ts");

  fn decl() -> String { format!("type {NAME} = {TYPE};") }

  fn name() -> String { NAME.into() }

  fn inline() -> String { TYPE.into() }

  fn dependencies() -> Vec<ts_rs::Dependency>
  where Self: 'static {
    vec![]
  }

  fn transparent() -> bool { false }
}

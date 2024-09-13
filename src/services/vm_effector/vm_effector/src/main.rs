#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main() {}

#[marine]
pub fn version() -> String {
    VERSION.to_string()
}

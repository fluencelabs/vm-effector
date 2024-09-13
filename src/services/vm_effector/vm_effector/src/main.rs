#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

const VERSION: &str = env!("CARGO_PKG_VERSION");

const RANDOM_ID: &str = "bafkreiei3vrjgjfbzsayj6jmvebbpxvuafv2ypfzatzm5zuufoh7aaqy5m";

pub fn main() {}

#[marine]
pub fn version() -> String {
    VERSION.to_string()
}


#[marine]
pub fn random_id() -> String {
    RANDOM_ID.to_string()
}

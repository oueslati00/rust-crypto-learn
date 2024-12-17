use uint::construct_uint;
construct_uint! {
    pub struct U256(4);
}
pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;
extern crate sha256 as sha256_lib;

extern crate serde;
extern crate ciborium;

use serde::{Deserialize, Serialize};
use uint::construct_uint;
construct_uint! {
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

pub const INITIAL_REWARD: u64 = 50;
pub const HALVING_INTERVAL: u64 = 210;
pub const IDEAL_BLOCK_TIME: u64 = 10;
pub const MIN_TARGET: U256 = U256([
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0xFFFF_FFFF_FFFF_FFFF,
    0x0000_FFFF_FFFF_FFFF,
]);
pub const DIFFICULTY_UPDATE_INTERVAL: u64 = 50;
pub const MAX_MEMPOOL_TRANSACTION_AGE: u64 = 600;
pub mod crypto;
pub mod sha256;
pub mod util;
pub mod types;
pub mod error;
pub mod network;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

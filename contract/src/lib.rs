//use methods::{ZK_PROVER_ELF, ZK_PROVER_ID};
use shared::types::{ZkCommit};
use near_sdk::{
    borsh::{self, BorshSerialize, BorshDeserialize},
    near_bindgen, env,
};
use risc0_zkvm::{
    Receipt,
    serde::from_slice,
};
use std::collections::HashMap;
use base64ct::{Base64, Encoding};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    my_state: String,
}

#[near_bindgen]
impl Contract {

    pub fn verify_zkp(&self, proof: String) -> bool {
        let receipt: Receipt = bincode::deserialize(&Base64::decode_vec(&proof).unwrap()).unwrap();
        let (verdict, error, journal) = match receipt.verify([4281092572, 1258533245, 3634752599, 2329801241, 608529344, 2747104430, 2014386172, 871482807]) {
            Ok(()) => {
                let journal: ZkCommit = from_slice(&receipt.journal).unwrap();
                (true, Option::None, Option::Some(journal))
            },
            Err(error) => (false, Option::Some(error.to_string()), Option::None),
        };

        return verdict;
    }
}
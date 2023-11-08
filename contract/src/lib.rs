use near_sdk::{
    borsh::{self, BorshSerialize, BorshDeserialize},
    near_bindgen, env,
};
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: HashMap<String, String>,
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(account_id.to_string(), message);
    }

    pub fn get_status(&self, account_id: String) -> Option<String> {
        self.records.get(&account_id).cloned()
    }
}
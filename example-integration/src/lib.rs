use near_sdk::{
    borsh::{self, BorshSerialize, BorshDeserialize},
    BorshStorageKey, near_bindgen, env, AccountId,
    serde_json::{self, json},
};
use std::collections::HashSet;
use shared::types::{CredentialReceiver, ZkCommit, CredentialSchema, CredentialHash, CredentialSchemaId, AcIssuer, AcHolder, AcRP};


//#[derive(BorshStorageKey, BorshSerialize)]
//pub(crate) enum StorageKey {
//    TrustedCredentialsKey,
//    WhitelistedUsersKey,
//}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    trusted_credentials: HashSet<(String, usize)>,
    trusted_script: String,
    whitelisted_users: HashSet<String>,
}

impl Default for Contract {
    fn default() -> Self {
        //let whitelisted_users = LookupSet::new(StorageKey::WhitelistedUsersKey);
        //let mut trusted_credentials = LookupSet::new(StorageKey::TrustedCredentialsKey);
        
        Self {
            whitelisted_users: HashSet::new(),
            trusted_credentials: HashSet::from_iter(vec![
                ("chluff.testnet".to_string(), 0),
                ("chluff.testnet".to_string(), 1),
                ("chluff.testnet".to_string(), 2),
                ("lennczar.testnet".to_string(), 0),
                ("lennczar.testnet".to_string(), 1),
                ("lennczar.testnet".to_string(), 2),
            ]),
            trusted_script: "credentials[0][\"age\"] >= 18;".to_string(),
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn get_whitelisted_users(&self) -> Vec<&String> {
        self.whitelisted_users.iter().collect()
    }
}

impl CredentialReceiver for Contract {
    fn on_cred_call (&mut self, holder: AcHolder, used_schemata: Vec<(AcIssuer, CredentialSchemaId)>, journal: ZkCommit) -> bool {
        assert!(journal.result, "The ZKP result must be true");
        assert!(journal.script == self.trusted_script, " Must use exactly our script");
        
        // check that the credentials are of trusted issuers
        let mut all_trusted = false;
        for (i, schema_info) in used_schemata.iter().enumerate() {
            if !self.trusted_credentials.contains(schema_info) {
                all_trusted = false;
                break;
            }
        }
        assert!(all_trusted, "All credentials must be issued by a trusted source");

        // All checks passed, whitelist the user!
        self.whitelisted_users.insert(holder)
    }
}


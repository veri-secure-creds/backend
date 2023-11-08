use near_sdk::{
    borsh::{self, BorshSerialize, BorshDeserialize},
    near_bindgen, env, AccountId, collections::{LookupSet, LookupMap},
};
use std::collections::HashMap;

type CredentialSchema = String; 
type CredentialHash = String;

type AcIssuer = AccountId;
type AcHolder = AccountId;
type AcRP = AccountId;


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    credentialSchemata: LookupMap<AcIssuer, LookupSet<CredentialSchema>>,
    credentialHashes: LookupMap<(AcIssuer, CredentialSchema), LookupSet<CredentialHashes>>
}


#[near_bindgen]
impl Contract {
    
    // TODO decide whether functions should be payable

    pub fn addCredentialSchema(&mut self, credentialSchema: CredentialSchema) {
        self.credentialSchemata.insert(env::predecessor_account_id(), credentialSchema);
    }

    pub fn modifyCredentialHashes(
        &mut self, 
        schemaId: CredentialSchemaId,
        add: Vec<CredentialHash>,
        remove: Vec<CredentialHash>
    ) {

    }

    // pub fn set_status(&mut self, message: String) {
    //     let account_id = env::signer_account_id();
    //     self.records.insert(account_id.to_string(), message);
    // }

    // pub fn get_status(&self, account_id: String) -> Option<String> {
    //     self.records.get(&account_id).cloned()
    // }
}
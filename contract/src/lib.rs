//use methods::{ZK_PROVER_ELF, ZK_PROVER_ID};
use shared::types::{ZkCommit};
use near_sdk::{
    borsh::{self, BorshSerialize, BorshDeserialize},
    BorshStorageKey, near_bindgen, env, AccountId, collections::{LookupSet, LookupMap},
};
use std::{collections::HashSet, ops::Sub};
use risc0_zkvm::{
    Receipt,
    serde::from_slice,
};
use base64ct::{Base64, Encoding};

type CredentialSchema = String; 
type CredentialHash = String;

type CredentialSchemaId = usize;

type AcIssuer = AccountId;
type AcHolder = AccountId;
type AcRP = AccountId;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    CredentialSchemaIdsKey,
    CredentialSchemataKey,
    CredentialHashesKey,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    credential_schema_ids: LookupMap<AcIssuer, HashSet<CredentialSchemaId>>,
    credential_schemata: LookupMap<(AcIssuer, CredentialSchemaId), CredentialSchema>,
    credential_hashes: LookupMap<(AcIssuer, CredentialSchemaId), HashSet<CredentialHash>>
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            credential_schema_ids: LookupMap::new(StorageKey::CredentialSchemaIdsKey),
            credential_schemata: LookupMap::new(StorageKey::CredentialSchemataKey),
            credential_hashes: LookupMap::new(StorageKey::CredentialHashesKey),
        }
    }
}


#[near_bindgen]
impl Contract {
    // TODO decide whether functions should be payable
    pub fn add_credential_schema(&mut self, credential_schema: CredentialSchema) {
        let issuer: AcIssuer = env::predecessor_account_id();
        let mut id: CredentialSchemaId = 0;

        if self.credential_schema_ids.contains_key(&issuer) {
            id = self.credential_schema_ids.get(&issuer).unwrap().len();
            self.credential_schema_ids.get(&issuer).unwrap().insert(id);
        } else {
            self.credential_schema_ids.insert(&issuer, &HashSet::from_iter(vec![id]));
        }

        self.credential_schemata.insert(&(issuer, id), &credential_schema);
    }

    pub fn modify_credential_hashes(
        &mut self, 
        schema_id: CredentialSchemaId,
        add: Vec<CredentialHash>,
        remove: Vec<CredentialHash>
    ) {
        let issuer: AcIssuer = env::predecessor_account_id();

        let issuers_ids = self.credential_schema_ids.get(&issuer).expect("Issuer does not have any registered schemata");
        assert!(issuers_ids.contains(&schema_id), "Unknown schema id");

        if !self.credential_hashes.contains_key(&(issuer.clone(), schema_id)) {
            self.credential_hashes.insert(&(issuer.clone(), schema_id), &HashSet::new());
        }

        let mut hashes = self.credential_hashes.get(&(issuer.clone(), schema_id)).expect("how did we get here?");
        hashes.extend(add);
        hashes = hashes.sub(&HashSet::from_iter(remove));
        self.credential_hashes.insert(&(issuer.clone(), schema_id), &hashes);
    }

    pub fn get_schema_ids(&self, issuer: AcIssuer) -> Vec<CredentialSchemaId> {
        self.credential_schema_ids.get(&issuer).expect("No schemata assigned to this issuer").into_iter().collect()
    }

    pub fn get_schema(&self, issuer: AcIssuer, schema_id: CredentialSchemaId) -> CredentialSchema {
        self.credential_schemata.get(&(issuer, schema_id)).expect("No schema with this id assigned to this issuer")
    }

    pub fn get_hashes(&self, issuer: AcIssuer, schema_id: CredentialSchemaId) -> Vec<CredentialHash> {
        self.credential_hashes.get(&(issuer, schema_id)).expect("No hashes for this schema id and issuer").into_iter().collect()
    }

    pub fn get_all_schemata(&self, issuer: AcIssuer) -> Vec<CredentialSchema> {
        let issuers_ids = self.credential_schema_ids.get(&issuer.clone()).expect("No schemata assigned to this issuer");
        issuers_ids.iter().map(|&id| self.credential_schemata.get(&(issuer.clone(), id)).unwrap()).collect()
    }

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
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct ZkCommit {
    pub has_error: bool,
    pub error_msg: String,
    pub cred_hashes: Vec<String>,
    pub pub_key: String,
    pub lang: ScriptLang,
    pub script: String,
    pub result: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ZkvmInput {
    pub credentials: Vec<String>,
    pub lang: ScriptLang,
    pub script: String,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum ScriptLang {
    Rhai,
    JavaScript,
}


pub type CredentialSchema = String; 
pub type CredentialHash = String;

pub type CredentialSchemaId = usize;

pub type AcIssuer = String;
pub type AcHolder = String;
pub type AcRP = String;

pub trait CredentialReceiver {
    fn on_cred_call (&mut self, holder: AcHolder, used_schemata: Vec<(AcIssuer, CredentialSchemaId)>, journal: ZkCommit) -> bool;
}

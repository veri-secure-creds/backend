use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
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

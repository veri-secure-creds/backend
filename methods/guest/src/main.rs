#![no_main]


use risc0_zkvm::{
    guest::env,
    sha::{self, Sha256},
};
use shared::types::{ZkCommit, ZkvmInput, ScriptLang};
use rhai::{Engine, Scope, Dynamic};
use serde_json::{Value, de::from_str};
use base64ct::{Base64, Encoding};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // error flag
    let mut has_error = false;

    let inputs: ZkvmInput = env::read();
    // get credentials
    let credentials: Vec<String> = inputs.credentials;
    // get script
    let script_lang: ScriptLang = inputs.lang;
    // get script
    let input_script: String = inputs.script;

    // validate that credentials are JSON objects
    for cred in credentials.iter() {
        let a = from_str::<Value>(cred);
        if a.is_err() { has_error = true; break; }
        else {
            match a.unwrap() {
                Value::Object(_) => (),
                _ => { has_error = true; break; }
            }   
        }
    }

    // stop if we found any errors
    if has_error {
        env::commit(&ZkCommit {
            has_error: true,
            err_msg: "failed to parse credentials".to_string(),
            cred_hashes: Vec::new(),
            lang: inputs.lang,
            script: input_script,
            result: false,
        });
        
        return;
    }

    // calculate sha256 hash of each credential
    let cred_hashes: Vec<String> = credentials
        .iter()
        .map(|cred| Base64::encode_string(sha::Impl::hash_bytes(cred.as_bytes()).as_bytes()))
        .collect();

    
    match script_lang {
        ScriptLang::Rhai => {
            let engine = Engine::new_raw();
            let mut scope = Scope::new();
            
            // inject credentials in the script
            let rhai_creds: Dynamic = credentials
                .iter()
                .map(|cred_str| engine.parse_json(cred_str, true).unwrap())
                .collect::<Vec<_>>()
                .into();
            scope.push_constant_dynamic("credentials", rhai_creds);
            
            // run the script
            let raw_result = engine.eval_with_scope::<bool>(&mut scope, &input_script);

            if raw_result.is_err() {
                env::commit(&ZkCommit {
                    has_error: true,
                    err_msg: format!("script error: {}", raw_result.err().unwrap()),
                    cred_hashes,
                    lang: inputs.lang,
                    script: input_script,
                    result: false,
                });
                
                return;
            }

            env::commit(&ZkCommit {
                has_error: false,
                err_msg: "".to_string(),
                cred_hashes,
                lang: inputs.lang,
                // IMPORTANT!! use input script here to not expose credentials
                script: input_script,
                result: raw_result.unwrap(),
            });
        },
        ScriptLang::JavaScript => (),
    }
}
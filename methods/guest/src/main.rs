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
    let mut error_msg = "".to_string();

    // pub_key disclosure (to prove on-chain that credential holder is the rightful owner to that credential)
    let mut pub_key = "".to_string();

    let inputs: ZkvmInput = env::read();
    // get credentials
    let credentials: Vec<String> = inputs.credentials;
    // get script
    let script_lang: ScriptLang = inputs.lang;
    // get script
    let input_script: String = inputs.script;

    // validate that credentials: 1) are JSON objects and 2) have "pub_key" attribute
    for (i, cred) in credentials.iter().enumerate() {
        let a = from_str::<Value>(cred);
        if a.is_err() {
            error_msg = "failed to parse Value from credential string".to_string();
            has_error = true;
            break;
        }
        else {
            match a.unwrap() {
                Value::Object(credential_object) => {
                    match credential_object.get("pub_key") {
                        Option::Some(value) => {
                            match value {
                                Value::String(pub_key_val) => {
                                    if i == 0 { pub_key = pub_key_val.clone() }
                                    else {
                                        if pub_key != *pub_key_val {
                                            error_msg = "Public mismatch between credentials".to_string();
                                            has_error = true;
                                            break;        
                                        }
                                    }
                                },
                                _ => {
                                    error_msg = "failed to parse public key value from credentials".to_string();
                                    has_error = true;
                                    break;
                                }
                            }
                        },
                        Option::None => {
                            error_msg = "failed to parse public key attribute key from credentials".to_string();
                            has_error = true;
                            break;
                        }
                    }
                },
                _ => {
                    error_msg = "failed to parse Object from credential string".to_string();
                    has_error = true;
                    break;
                }
            }
        }
    }


    // stop if we found any errors
    if has_error {
        env::commit(&ZkCommit {
            has_error: true,
            error_msg,
            cred_hashes: Vec::new(),
            pub_key,
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
                    error_msg: format!("script error: {}", raw_result.err().unwrap()),
                    cred_hashes,
                    pub_key,
                    lang: inputs.lang,
                    script: input_script,
                    result: false,
                });
                
                return;
            }

            env::commit(&ZkCommit {
                has_error: false,
                error_msg: "".to_string(),
                cred_hashes,
                pub_key,
                lang: inputs.lang,
                // IMPORTANT!! use input script here to not expose credentials
                script: input_script,
                result: raw_result.unwrap(),
            });
        },
        ScriptLang::JavaScript => (),
    }
}
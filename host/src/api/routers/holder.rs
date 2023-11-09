// TODO: proper API lifecycle
// TODO: Simulate result with rhai engine to validate the script works correctly before trying to geenerate a zk-proof
// TODO: compile outside of zkVM, only send serialized AST as input
//let rhai_engine = Engine::new_raw();
//let rhai_ast = rhai_engine.compile(rhai_code).unwrap();


use methods::{ZK_PROVER_ELF, ZK_PROVER_ID};
use shared::types::{ZkCommit, ZkvmInput, ScriptLang};
use risc0_zkvm::{
    Executor, ExecutorEnv,
    serde::{to_vec, from_slice},
};
use std::{time::Instant, collections::HashMap, sync::Arc};
use rhai::Engine;
use axum::{
    routing::{Router, post, get},
    Json, http::StatusCode,
    extract::{State, Path},
};
use serde::{Serialize, Deserialize};
use serde_json::ser::to_string;
use base64ct::{Base64, Encoding};
use std::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    data: Arc<Mutex<SharedData>>
}

#[derive(Clone)]
pub struct SharedData {
    pub task_id: usize,
    pub is_running: bool,
    pub results: HashMap<usize, GetProofResponse>
}


#[derive(Serialize, Clone)]
pub struct GetProofResponse {
    proof: String,
    journal: Option<ZkCommit>,
}

#[derive(Deserialize)]
pub struct GenProofArgs {
    credentials: Vec<String>,
    script: String,
}

pub fn holder_router() -> Router {
    let state = AppState{
        data: Arc::new(Mutex::new(SharedData {
            task_id: 0,
            is_running: false,
            results: HashMap::new(),
        }))
    };

    Router::new()
        .route("/genproof", post(gen_proof))
        .route("/getproof/:task_id", get(get_proof))
        .with_state(state)
}

pub async fn gen_proof(State(state): State<AppState>, Json(payload): Json<GenProofArgs>) -> (StatusCode, Json<isize>) {
    let task_id: isize = {
        let mut data = state.data.lock().expect("mutex was poisoned");
        let return_val: isize = if data.is_running {
            // We are busy: return task ID -1
            -1
        } else {
            data.task_id += 1;
            data.is_running = true;
            data.task_id.try_into().unwrap()
        };

        return_val
    };

    if task_id == -1 { return ( StatusCode::ACCEPTED, Json(task_id)) }

    tokio::spawn(async move {
        // First, we construct an executor environment
        let env = ExecutorEnv::builder()
        .add_input(&to_vec(&ZkvmInput {
            credentials: payload.credentials,
            lang: ScriptLang::Rhai,
            script: payload.script,
        }).unwrap())
        .build()
        .unwrap();

        // Next, we make an executor, loading the (renamed) ELF binary.
        let mut exec = Executor::from_elf(env, ZK_PROVER_ELF).unwrap();

        // Run the executor to produce a session.
        let session = exec.run().unwrap();
        println!("Starting proof generation...");
        let start_time_prover = Instant::now();

        // Prove the session to produce a receipt.
        let receipt = session.prove().unwrap();

        println!("Prover duration: {:?}m {:?}s", start_time_prover.elapsed().as_secs() / 60, start_time_prover.elapsed().as_secs() % 60);
        println!("Receipt size {:.2} (KB)", (to_vec(&receipt).unwrap().len() / 1024));

        // Get guest result
        let code_result: ZkCommit = from_slice(&receipt.journal).unwrap();
        println!("Result: {:?}", to_string(&code_result));

        // Verify receipt to confirm that recipients will also be able to verify it
        let start_time_verifier = Instant::now();
        receipt.verify(ZK_PROVER_ID).unwrap();
        println!("Verifier duration {:?}", start_time_verifier.elapsed());
        
        {
            let mut data = state.data.lock().expect("mutex was poisoned");
            let task_id = data.task_id;
            data.is_running = false;
            data.results.insert(
                task_id,
                GetProofResponse {
                    proof: Base64::encode_string(&bincode::serialize(&receipt).unwrap()),
                    journal: Option::Some(from_slice(&receipt.journal).unwrap()),
                }
            );
        }
    });

    (
        StatusCode::ACCEPTED,
        Json(task_id)
    )
}

pub async fn get_proof(State(state): State<AppState>, Path(task_id): Path<usize>) -> (StatusCode, Json<GetProofResponse>) {
    let response: GetProofResponse = {
        let data = state.data.lock().expect("mutex was poisoned");
        data.results.get(&task_id).unwrap_or(&GetProofResponse {
            proof: "".to_string(),
            journal: Option::None,
        }).clone()
    };
    (StatusCode::ACCEPTED, Json(response))
}

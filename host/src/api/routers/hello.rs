use axum::{
    Router,
    routing::get,
};

pub fn hello_router() -> Router {
    Router::new()
        .route("/", get(handle_hello))
}

// basic handler that responds with a static string
async fn handle_hello() -> String {
    "Hello, World!".to_string()
}
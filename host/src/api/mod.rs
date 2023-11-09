mod routers;

use axum::{Router, http::Method};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use routers::{
    hello::hello_router,
    holder::holder_router,
};
use tower_http::cors::{CorsLayer, Any};


pub async fn api_start() {
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .nest("/hello", hello_router())
        .nest("/holder", holder_router())
        .layer(cors);

    let addr = SocketAddr::from((
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        3000
    ));
    println!("listening on {}", addr);
    
    let app = Router::new().nest("/", api_routes);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
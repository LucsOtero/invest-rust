mod db;
mod auth;
mod models;
mod handlers;
mod dashboard;

use axum::{
    routing::{get, post},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/login", post(handlers::login))
        .route("/investments", get(handlers::list_investments))
        .route("/investments/create", post(handlers::create_investment))
        .route("/investments/delete", post(handlers::delete_investment));

    let addr = "127.0.0.1:3000";
    println!("Servidor rodando em {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

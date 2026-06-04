mod db;
mod auth;
mod models;
mod handlers;
mod dashboard;

use axum::{routing::{get, post}, Router};
use tower_cookies::CookieManagerLayer;
use std::env;

#[tokio::main]
async fn main() {
    // Carrega variáveis do arquivo .env
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("A variável DATABASE_URL precisa existir");

    // Conecta com o PostgreSQL
    let pool = db::connect(&db_url).await;

    // Configura o Axum com Estado (Banco) e Cookies
    let app = Router::new()
        .route("/login", post(handlers::login))
        .route("/investments", get(handlers::list_investments))
        .route("/investments/create", post(handlers::create_investment))
        .with_state(pool)
        .layer(CookieManagerLayer::new());

    let addr = "127.0.0.1:3000";
    println!("Servidor rodando e conectado ao banco na porta 3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

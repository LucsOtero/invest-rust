use axum::{Json, routing::{get, post}, Router};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub async fn login(Json(payload): Json<Login>) -> String {
    crate::auth::create_token(payload.email)
}

pub async fn list_investments() -> String {
    "Lista de investimentos".to_string()
}

pub async fn create_investment() -> String {
    "Investimento criado".to_string()
}

pub async fn delete_investment() -> String {
    "Investimento deletado".to_string()
}

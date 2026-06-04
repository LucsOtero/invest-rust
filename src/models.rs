use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use askama::Template;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Investment {
    pub id: i32,
    pub user_id: Option<i32>,
    pub asset: String,
    pub quantity: f64,
    pub price: f64,
}

// Estrutura que conecta o Rust com o arquivo HTML
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub total_value: f64,
    pub investments: Vec<Investment>,
}

use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use askama::Template;

// Estrutura que representa o usuário no banco de dados
#[derive(Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: i32,
    pub email: String,
    pub senha_hash: String,
}

// Representa cada ativo comprado na bolsa ou renda fixa
#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Ativo {
    pub id: i32,
    pub id_usuario: Option<i32>,
    pub codigo_ativo: String,
    pub qtd_cotas: f64,
    pub preco_compra: f64,
}

// Estrutura que liga o Rust com a nossa tela HTML
#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct TelaDashboard {
    pub valor_total_carteira: f64,
    pub lista_ativos: Vec<Ativo>,
}

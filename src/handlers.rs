use axum::{extract::State, response::Html, Json};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};
use serde::Deserialize;
use askama::Template;
use crate::models::{Investment, DashboardTemplate};

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct NewInvestment {
    pub user_id: i32,
    pub asset: String,
    pub quantity: f64,
    pub price: f64,
}

// 1. JWT E COOKIES: Gera o token e salva no navegador
pub async fn login(cookies: Cookies, Json(payload): Json<Login>) -> String {
    let token = crate::auth::create_token(payload.email);
    cookies.add(Cookie::new("jwt", token));
    "Login efetuado e Cookie salvo com sucesso!".to_string()
}

// 2. PERSISTÊNCIA SQLX: Salva o dado real no PostgreSQL
pub async fn create_investment(
    State(pool): State<PgPool>,
    Json(payload): Json<NewInvestment>,
) -> String {
    sqlx::query(
        "INSERT INTO investments (user_id, asset, quantity, price) VALUES ($1, $2, $3, $4)"
    )
    .bind(payload.user_id)
    .bind(payload.asset)
    .bind(payload.quantity)
    .bind(payload.price)
    .execute(&pool)
    .await
    .unwrap();

    "Investimento inserido no banco de dados!".to_string()
}

// 3. ASKAMA + DASHBOARD: Busca no banco, calcula e renderiza o HTML
pub async fn list_investments(State(pool): State<PgPool>) -> Html<String> {
    let invs = sqlx::query_as::<_, Investment>("SELECT * FROM investments")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let calc_data: Vec<(f64, f64)> = invs.iter().map(|i| (i.quantity, i.price)).collect();
    let total = crate::dashboard::calculate_total(calc_data);

    let template = DashboardTemplate {
        total_value: total,
        investments: invs,
    };

    Html(template.render().unwrap())
}

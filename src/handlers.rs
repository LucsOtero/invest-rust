use axum::{extract::State, response::Html, Json};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};
use serde::Deserialize;
use askama::Template;
use crate::models::{Ativo, TelaDashboard};

#[derive(Deserialize)]
pub struct DadosLogin {
    pub email: String,
    pub senha_hash: String,
}

#[derive(Deserialize)]
pub struct NovoAtivo {
    pub id_usuario: i32,
    pub codigo_ativo: String,
    pub qtd_cotas: f64,
    pub preco_compra: f64,
}

// Rota responsável por logar o usuário e salvar o cookie no navegador
pub async fn login(cookies: Cookies, Json(dados): Json<DadosLogin>) -> String {
    let token_gerado = crate::auth::create_token(dados.email);
    cookies.add(Cookie::new("meu_token_jwt", token_gerado));
    "Usuário autenticado com sucesso!".to_string()
}

// Rota que cadastra um novo investimento direto no PostgreSQL
pub async fn create_investment(
    State(banco): State<PgPool>,
    Json(novo_dado): Json<NovoAtivo>,
) -> String {
    // Inserindo os dados de forma segura para evitar SQL Injection
    sqlx::query(
        "INSERT INTO meus_ativos (id_usuario, codigo_ativo, qtd_cotas, preco_compra) VALUES ($1, $2, $3, $4)"
    )
    .bind(novo_dado.id_usuario)
    .bind(novo_dado.codigo_ativo)
    .bind(novo_dado.qtd_cotas)
    .bind(novo_dado.preco_compra)
    .execute(&banco)
    .await
    .unwrap();

    "Ativo salvo na sua carteira!".to_string()
}

// Rota que busca os dados, faz o cálculo e monta a tela pro usuário
pub async fn list_investments(State(banco): State<PgPool>) -> Html<String> {
    // Busca tudo que tem na tabela meus_ativos
    let ativos_db = sqlx::query_as::<_, Ativo>("SELECT * FROM meus_ativos")
        .fetch_all(&banco)
        .await
        .unwrap_or_default();

    // Separa apenas a quantidade e o preço para a função de calcular
    let dados_para_calculo: Vec<(f64, f64)> = ativos_db.iter().map(|a| (a.qtd_cotas, a.preco_compra)).collect();
    let total_calculado = crate::dashboard::calculate_total(dados_para_calculo);

    // Manda as informações prontas para o Askama renderizar o HTML
    let minha_tela = TelaDashboard {
        valor_total_carteira: total_calculado,
        lista_ativos: ativos_db,
    };

    Html(minha_tela.render().unwrap())
}

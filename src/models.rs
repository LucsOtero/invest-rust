use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Investment {
    pub id: i32,
    pub user_id: i32,
    pub asset: String,
    pub quantity: f64,
    pub price: f64,
}

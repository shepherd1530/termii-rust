use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceItem {
    pub user: String,
    pub currency: String,
    pub balance: f64,
}

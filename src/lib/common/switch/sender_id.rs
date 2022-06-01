use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderIDResponse {
    current_page: i64,
    pub(crate) data: Vec<SenderIDItem>,
    first_page_url: String,
    from: Option<i64>,
    last_page: i64,
    last_page_url: String,
    next_page_url: Option<String>,
    path: String,
    per_page: i64,
    prev_page_url: Option<String>,
    to: Option<i64>,
    total: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderIDItem {
    pub sender_id: String,
    pub status: String,
    pub company: Option<String>,
    pub usecase: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderIDRequest {
    pub sender_id: String,
    pub usecase: String,
    pub company: String,
    api_key: Option<String>,
}

impl SenderIDRequest {
    pub fn new(sender_id: String, usecase: String, company: String) -> SenderIDRequest {
        SenderIDRequest {
            sender_id,
            usecase,
            company,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderIDRequestResponse {
    pub code: String,
    pub message: String,
}

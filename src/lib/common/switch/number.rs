use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NumberMessageResponse {
    pub code: String,
    pub message_id: String,
    pub message: String,
    pub balance: f64,
    pub user: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NumberMessageRequest {
    pub to: String,
    pub sms: String,
    api_key: Option<String>,
}

impl NumberMessageRequest {
    pub fn new(to: String, sms: String) -> NumberMessageRequest {
        NumberMessageRequest {
            to,
            sms,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

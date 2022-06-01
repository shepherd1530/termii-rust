use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum InAppTokenMessageType {
    NUMERIC,
    ALPHANUMERIC,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InAppTokenRequest {
    pub phone_number: String,
    pub pin_type: InAppTokenMessageType,
    pub pin_attempts: u8,
    pub pin_time_to_live: usize,
    pub pin_length: u8,
    api_key: Option<String>,
}

impl InAppTokenRequest {
    pub fn new(
        phone_number: String,
        pin_type: InAppTokenMessageType,
        pin_attempts: u8,
        pin_time_to_live: usize,
        pin_length: u8,
    ) -> InAppTokenRequest {
        InAppTokenRequest {
            phone_number,
            pin_type,
            pin_attempts,
            pin_time_to_live,
            pin_length,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InAppTokenResponse {
    pub status: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub pin_id: String,
    pub otp: String,
    pub phone_number: String,
    pub phone_number_other: String,
}

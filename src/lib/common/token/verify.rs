use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyTokenRequest {
    pub pin_id: String,
    pub pin: String,
    api_key: Option<String>,
}

impl VerifyTokenRequest {
    pub fn new(pin_id: String, pin: String) -> VerifyTokenRequest {
        VerifyTokenRequest {
            pin_id,
            pin,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyTokenResponse {
    #[serde(rename = "pinId")]
    pub pin_id: String,
    pub verified: bool,
    pub msisdn: String,
}

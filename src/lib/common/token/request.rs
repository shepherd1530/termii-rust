use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestTokenMessageType {
    NUMERIC,
    ALPHANUMERIC,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestTokenPinType {
    NUMERIC,
    ALPHANUMERIC,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestTokenChannel {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "dnd")]
    Dnd,
    #[serde(rename = "whatsapp")]
    Whatsapp,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestTokenRequest {
    pub message_type: RequestTokenMessageType,
    pub to: String,
    pub from: String,
    pub channel: RequestTokenChannel,
    pub pin_attempts: u8,
    pub pin_time_to_live: usize,
    pub pin_length: u8,
    pub pin_placeholder: String,
    pub message_text: String,
    pub pin_type: RequestTokenPinType,
    api_key: Option<String>,
}

impl RequestTokenRequest {
    pub fn new(
        message_type: RequestTokenMessageType,
        to: String,
        from: String,
        channel: RequestTokenChannel,
        pin_attempts: u8,
        pin_time_to_live: usize,
        pin_length: u8,
        pin_placeholder: String,
        message_text: String,
        pin_type: RequestTokenPinType,
    ) -> RequestTokenRequest {
        RequestTokenRequest {
            message_type,
            to,
            from,
            channel,
            pin_attempts,
            pin_time_to_live,
            pin_length,
            pin_placeholder,
            message_text,
            pin_type,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestTokenResponse {
    #[serde(rename = "pinId")]
    pub pin_id: String,
    pub to: String,
    #[serde(rename = "smsStatus")]
    pub sms_status: String,
}

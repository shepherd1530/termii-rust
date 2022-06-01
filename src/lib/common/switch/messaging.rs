use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageResponse {
    #[serde(deserialize_with = "from_usize")]
    pub message_id: String,
    pub message: String,
    pub balance: f64,
    pub user: String,
    pub code: Option<String>,
}

fn from_usize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: usize = Deserialize::deserialize(deserializer)?;
    Ok(s.to_string())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageRequest {
    pub to: String,
    pub from: String,
    pub sms: String,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub channel: Channel,
    pub media: Option<Media>,
    api_key: Option<String>,
}

impl MessageRequest {
    pub fn new(
        to: String,
        from: String,
        sms: String,
        message_type: MessageType,
        channel: Channel,
    ) -> MessageRequest {
        MessageRequest {
            to,
            from,
            sms,
            message_type,
            channel,
            media: None,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBulkRequest {
    pub to: Vec<String>,
    pub from: String,
    pub sms: String,
    #[serde(rename = "type")]
    pub message_type: MessageType,
    pub channel: Channel,
    api_key: Option<String>,
}

impl MessageBulkRequest {
    pub fn new(
        to: Vec<String>,
        from: String,
        sms: String,
        message_type: MessageType,
        channel: Channel,
    ) -> MessageBulkRequest {
        MessageBulkRequest {
            to,
            from,
            sms,
            message_type,
            channel,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBulkResponse {
    pub code: String,
    pub message_id: String,
    pub message: String,
    pub balance: f64,
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub url: String,
    pub caption: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    #[serde(rename = "plain")]
    Plain,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Channel {
    #[serde(rename = "generic")]
    Generic,
    #[serde(rename = "dnd")]
    Dnd,
    #[serde(rename = "whatsapp")]
    Whatsapp,
}

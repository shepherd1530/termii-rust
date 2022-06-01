use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryItem {
    pub sender: String,
    pub receiver: String,
    pub message: String,
    pub amount: usize,
    pub reroute: usize,
    pub status: String,
    pub sms_type: String,
    pub send_by: String,
    pub media_url: Option<String>,
    pub message_id: String,
    pub notify_url: Option<String>,
    pub notify_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HistoryResponse {
    pub(crate) data: HistoryResponseData,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct HistoryResponseData {
    current_page: i64,
    pub(crate) data: Vec<HistoryItem>,
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

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplatesData {
    pub product_name: String,
    pub otp: String,
    pub expiry_time: String,
}

impl TemplatesData {
    pub fn new(product_name: String, otp: String, expiry_time: String) -> TemplatesData {
        TemplatesData {
            product_name,
            otp,
            expiry_time,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplatesRequest {
    pub phone_number: String,
    pub device_id: String,
    pub template_id: String,
    pub data: TemplatesData,
    api_key: Option<String>,
}

impl TemplatesRequest {
    pub fn new(
        phone_number: String,
        device_id: String,
        template_id: String,
        data: TemplatesData,
    ) -> TemplatesRequest {
        TemplatesRequest {
            phone_number,
            device_id,
            template_id,
            data,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TemplateItem {
    code: String,
    message_id: String,
    message: String,
    balance: String,
    user: String,
}

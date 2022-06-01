use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneBookListResponse {
    pub data: Vec<PhoneBookItem>,
    pub links: Links,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneBookItem {
    pub id: String,
    pub name: String,
    pub total_number_of_contacts: i64,
    pub date_created: String,
    pub last_updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {
    pub first: String,
    pub last: String,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub current_page: i64,
    pub from: Option<i64>,
    pub last_page: i64,
    pub path: String,
    pub per_page: i64,
    pub to: Option<i64>,
    pub total: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneBookCreateUpdateDeleteResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PhoneBookCreateUpdateRequest {
    pub phonebook_name: String,
    pub description: String,
    pub api_key: Option<String>,
}

impl PhoneBookCreateUpdateRequest {
    pub fn new(phonebook_name: String, description: String) -> PhoneBookCreateUpdateRequest {
        PhoneBookCreateUpdateRequest {
            phonebook_name,
            description,
            api_key: None,
        }
    }

    pub(crate) fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }
}

//! The search API verifies and detects a phonenumber's DND status.

use std::{collections::HashMap, sync::Arc};

use crate::{
    async_impl::http::client,
    common::{errors, insights::search::SearchItem},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Search<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> Search<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Search<'a> {
        Search { api_key, client }
    }

    /// Verify a phone number.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::insights::search::SearchItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let search:SearchItem = client.insights.search.get("234XXXXXXXXXX").await.unwrap();
    ///
    /// println!("{:?}", search);
    /// ```
    pub async fn get(&self, phone_number: &str) -> Result<SearchItem, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("phone_number", phone_number);

        let response = self.client.get("check/dnd", Some(params), None).await?;

        let search_item = response_or_error_text_async!(response, SearchItem);

        Ok(search_item)
    }
}

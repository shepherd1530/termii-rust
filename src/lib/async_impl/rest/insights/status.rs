//! The status API detects fake or ported phone numbers.

use std::{collections::HashMap, sync::Arc};

use crate::{
    async_impl::http::client,
    common::{errors, insights::status::StatusItem},
};

#[derive(Debug)]
pub struct Status<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> Status<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Status<'a> {
        Status { api_key, client }
    }

    /// Detects fake or ported numbers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::insights::status::StatusItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let status:StatusItem = client.insights.status.get("234XXXXXXXXXX", "NG").await.unwrap();
    ///
    /// println!("{:?}", status);
    /// ```
    pub async fn get(
        &self,
        phone_number: &str,
        country_code: &str,
    ) -> Result<StatusItem, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("phone_number", phone_number);
        params.insert("country_code", country_code);

        let response = self
            .client
            .get("insight/number/query", Some(params), None)
            .await?;

        let status_response = response_or_error_text_async!(response, StatusItem);

        Ok(status_response)
    }
}

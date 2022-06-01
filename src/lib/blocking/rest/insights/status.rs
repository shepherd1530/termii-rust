//! The status API detects fake or ported phone numbers.

use std::{collections::HashMap, rc::Rc};

use crate::{
    blocking::http::client,
    common::{errors, insights::status::StatusItem},
};

#[derive(Debug)]
pub struct Status<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
}

impl<'a> Status<'a> {
    pub(crate) fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> Status<'a> {
        Status { api_key, client }
    }

    /// Detects fake or ported numbers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii,
    ///     common::insights::status::StatusItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let status:StatusItem = client.insights.status.get("234XXXXXXXXXX", "NG").unwrap();
    ///
    /// println!("{:?}", status);
    /// ```
    pub fn get(
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
            .get("insight/number/query", Some(params), None)?;

        let status_response = response_or_error_text_blocking!(response, StatusItem);

        Ok(status_response)
    }
}

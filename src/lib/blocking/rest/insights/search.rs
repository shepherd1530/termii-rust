//! The search API

use std::{collections::HashMap, rc::Rc};

use crate::{
    blocking::http::client,
    common::{errors, insights::search::SearchItem},
};

#[derive(Debug)]
pub struct Search<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
}

impl<'a> Search<'a> {
    pub(crate) fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> Search<'a> {
        Search { api_key, client }
    }

    /// Verify a phone number.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii,
    ///     common::insights::search::SearchItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let search:SearchItem = client.insights.search.get("234XXXXXXXXXX").unwrap();
    ///
    /// println!("{:?}", search);
    /// ```
    pub fn get(&self, phone_number: &str) -> Result<SearchItem, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("phone_number", phone_number);

        let response = self.client.get("check/dnd", Some(params), None)?;

        let search_item = response_or_error_text_blocking!(response, SearchItem);

        Ok(search_item)
    }
}

//! Retrieve your org's wallet balance via API.

use std::{collections::HashMap, rc::Rc};

use crate::{
    blocking::http::client,
    common::{errors, insights::balance::BalanceItem},
};

#[derive(Debug)]
pub struct Balance<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
}

impl<'a> Balance<'a> {
    pub(crate) fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> Balance<'a> {
        Balance { api_key, client }
    }

    /// Gets your account balance.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii,
    ///     common::insights::balance::BalanceItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let balance:BalanceItem = client.insights.balance.get().unwrap();
    ///
    /// println!("{:?}", balance);
    /// ```
    pub fn get(&self) -> Result<BalanceItem, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);

        let response = self.client.get("get-balance", Some(params), None)?;

        let balance_item = response_or_error_text_blocking!(response, BalanceItem);

        Ok(balance_item)
    }
}

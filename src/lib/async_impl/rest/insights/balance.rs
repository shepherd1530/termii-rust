//! Retrieve your org's wallet balance via API.

use std::{collections::HashMap, sync::Arc};

use crate::{
    async_impl::http::client,
    common::{errors, insights::balance::BalanceItem},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Balance<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> Balance<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Balance<'a> {
        Balance { api_key, client }
    }

    /// Gets your account balance.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::insights::balance::BalanceItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let balance = client.insights.balance.get().await.unwrap();
    ///
    /// println!("{:?}", balance);
    /// ```
    pub async fn get(&self) -> Result<BalanceItem, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);

        let response = self.client.get("get-balance", Some(params), None).await?;

        let balance_item = response_or_error_text_async!(response, BalanceItem);

        Ok(balance_item)
    }
}

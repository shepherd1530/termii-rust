//! Request organization's account messaging history.

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        insights::history::{HistoryItem, HistoryResponse},
        pagination,
    },
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct History<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> History<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> History<'a> {
        History { api_key, client }
    }

    pub(crate) async fn _get(&self, page: &str) -> Result<Vec<HistoryItem>, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("page", page);

        let response = self.client.get("sms/inbox", Some(params), None).await?;

        let history_item = response_or_error_text_async!(response, HistoryResponse);

        Ok(history_item.data.data)
    }

    /// Gets your messaging history.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::insights::history::HistoryItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let history:HistoryItem = client.insights.history.get().await.unwrap();
    ///
    /// println!("{:?}", history);
    /// ```
    ///
    /// ### The above code is limited by termii's pagination. You can get all your messaging history with the **all** function like such
    /// ```rust
    /// let history = client.insights.history.all().await.unwrap();
    /// ```
    pub async fn get(&self, page: Option<&str>) -> Result<Vec<HistoryItem>, errors::HttpError> {
        let page = page.unwrap_or("1");
        let history_items = self._get(page).await?;
        Ok(history_items)
    }
}

#[async_trait]
impl pagination::PaginatedResourceAsync for History<'_> {
    type Item = HistoryItem;

    async fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError> {
        History::_get(self, page).await
    }
}

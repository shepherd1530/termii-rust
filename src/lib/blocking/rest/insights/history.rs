//! Request organization's account messaging history.

use std::{collections::HashMap, rc::Rc};

use crate::{
    blocking::http::client,
    common::{
        errors,
        insights::history::{HistoryItem, HistoryResponse},
        pagination,
    },
};

#[derive(Debug)]
pub struct History<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
}

impl<'a> History<'a> {
    pub(crate) fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> History<'a> {
        History { api_key, client }
    }

    pub(crate) fn _get(&self, page: &str) -> Result<Vec<HistoryItem>, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("page", page);

        let response = self.client.get("sms/inbox", Some(params), None)?;

        let history_item = response_or_error_text_blocking!(response, HistoryResponse);

        Ok(history_item.data.data)
    }

    /// Gets your messaging history.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii,
    ///     common::insights::history::HistoryItem,
    /// }
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let history:HistoryItem = client.insights.history.get().unwrap();
    ///
    /// println!("{:?}", history);
    /// ```
    ///
    /// ### The above code is limited by termii's pagination. You can get all your messaging history with the **all** function like such
    /// ```rust
    /// let history = client.insights.history.all().unwrap();
    /// ```
    pub fn get(&self, page: Option<&str>) -> Result<Vec<HistoryItem>, errors::HttpError> {
        let page = page.unwrap_or("1");
        let history_items = self._get(page)?;
        Ok(history_items)
    }
}

impl pagination::PaginatedResource for History<'_> {
    type Item = HistoryItem;

    fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError> {
        History::_get(self, page)
    }
}

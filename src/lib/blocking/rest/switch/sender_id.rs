//! Request new Sender Id and retrieve their status.

use std::{collections::HashMap, rc::Rc};

use crate::{
    blocking::http::client,
    common::{
        errors, pagination,
        switch::sender_id::{
            SenderIDItem, SenderIDRequest, SenderIDRequestResponse, SenderIDResponse,
        },
    },
};

#[derive(Debug)]
pub struct SenderID<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
}

impl<'a> SenderID<'a> {
    pub(crate) fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> SenderID<'a> {
        SenderID { api_key, client }
    }

    pub(crate) fn _get(&self, page: &str) -> Result<Vec<SenderIDItem>, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("page", page);

        let response = self.client.get("sender-id", Some(params), None)?;

        let sender_id_item = response_or_error_text_blocking!(response, SenderIDResponse);

        Ok(sender_id_item.data)
    }

    /// Fetch your organization's sender ID's.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::blocking::rest::termii;
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let sender_id = client.switch.sender_id.get(Some("1")).unwrap();
    ///
    /// println!("{:?}", sender_id);
    /// ```
    /// ### The above code is limited by termii's pagination. You can get all your sender ID's with the **all** function like such
    ///
    /// ```rust
    /// let sender_ids = client.switch.sender_id.all().unwrap();
    /// ```
    pub fn get(&self, page: Option<&str>) -> Result<Vec<SenderIDItem>, errors::HttpError> {
        let page = page.unwrap_or("1");
        let sender_id_items = self._get(page)?;
        Ok(sender_id_items)
    }

    /// Request a sender ID.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{blocking::rest::termii, common::switch::sender_id::SenderIDRequest};
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let sender_id_payload = SenderIDRequest::new(
    ///     "OrgNewsLetter".to_string(),
    ///     "New offering available.".to_string(),
    ///     "Your org name".to_string(),
    /// );
    ///
    /// let sender_id_response = client.switch.sender_id.request(sender_id_payload).unwrap();
    ///
    /// println!("{:?}", sender_id_response);
    /// ```
    pub fn request(
        &self,
        mut payload: SenderIDRequest,
    ) -> Result<SenderIDRequestResponse, errors::HttpError> {
        payload.set_api_key(self.api_key);

        let response = self
            .client
            .post("sender-id/request", None, None, Some(payload))?;

        let sender_id_request_response =
            response_or_error_text_blocking!(response, SenderIDRequestResponse);

        Ok(sender_id_request_response)
    }
}

impl pagination::PaginatedResource for SenderID<'_> {
    type Item = SenderIDItem;

    fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError> {
        SenderID::_get(self, page)
    }
}

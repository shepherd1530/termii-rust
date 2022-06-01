//! Send and manage campaigns sent to your organization's phonebook.
//!
//! Create, view & manage phonebooks using these APIs.
//!
//! Each phonebook can be identified by a unique ID, which makes it easier to edit or delete a phonebook.

use std::{collections::HashMap, rc::Rc};

use crate::{
    blocking::http::client,
    common::{
        errors, pagination,
        switch::campaign::{
            PhoneBookCreateUpdateDeleteResponse, PhoneBookCreateUpdateRequest, PhoneBookItem,
            PhoneBookListResponse,
        },
    },
};

#[derive(Debug)]
pub struct Campaign<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
}

impl<'a> Campaign<'a> {
    pub(crate) fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> Campaign<'a> {
        Campaign { api_key, client }
    }

    pub(crate) fn _get(&self, page: &str) -> Result<Vec<PhoneBookItem>, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("page", page);

        let response = self.client.get("phonebooks", Some(params), None)?;

        let campaign_item = response_or_error_text_blocking!(response, PhoneBookListResponse);

        Ok(campaign_item.data)
    }

    /// Fetch Phonebooks.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{blocking::rest::termii, common::switch::campaign::PhoneBookItem};
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let phonebooks: Vec<PhoneBookItem> = client.switch.campaign.get(Some(1)).unwrap();
    ///
    /// println!("{:?}", phonebooks);
    /// ```
    /// ### The above code is limited by termii's pagination. You can get all your phonebooks with the **all** function like such
    ///
    /// ```rust
    /// let phonebooks: Vec<PhoneBookItem> = client.switch.campaign.all().unwrap();
    /// ```
    pub fn get(&self, page: Option<&str>) -> Result<Vec<PhoneBookItem>, errors::HttpError> {
        let page = page.unwrap_or("1");
        let campaign_items = self._get(page)?;
        Ok(campaign_items)
    }

    /// Fetch Phonebooks.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii,
    ///     common::switch::campaign::{
    ///         PhoneBookCreateUpdateDeleteResponse, PhoneBookCreateUpdateRequest,
    ///     },
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let phonebook_create_request = PhoneBookCreateUpdateRequest {
    ///     phonebook_name: "My Phonebook".to_string(),
    ///     description: "My Phonebook".to_string(),
    /// };
    ///
    /// let phonebook_create_response: PhoneBookCreateUpdateDeleteResponse = client
    ///     .switch
    ///     .campaign
    ///     .create(phonebook_create_request)
    ///     .unwrap();
    ///
    /// println!("{:?}", phonebook_create_response);
    /// ```
    pub fn create(
        &self,
        mut payload: PhoneBookCreateUpdateRequest,
    ) -> Result<PhoneBookCreateUpdateDeleteResponse, errors::HttpError> {
        payload.set_api_key(self.api_key);

        let response = self.client.post("phonebooks", None, None, Some(payload))?;

        let campaign_create_response =
            response_or_error_text_blocking!(response, PhoneBookCreateUpdateDeleteResponse);

        Ok(campaign_create_response)
    }

    /// Update Phonebook.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii,
    ///     common::switch::campaign::{
    ///         PhoneBookCreateUpdateDeleteResponse, PhoneBookCreateUpdateRequest,
    ///     },
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let phonebook_update_request = PhoneBookCreateUpdateRequest {
    ///     phonebook_name: "My Phonebook".to_string(),
    ///     description: "My Phonebook".to_string(),
    /// };
    ///
    /// let phonebook_update_response: PhoneBookCreateUpdateDeleteResponse = client
    ///     .switch
    ///     .campaign
    ///     .update(
    ///         "f9c28de9-ab5a-4513-9c9f-338be8e1c390",
    ///         phonebook_update_request,
    ///     )
    ///     .unwrap();
    ///
    /// println!("{:?}", phonebook_update_response);
    /// ```
    pub fn update(
        &self,
        phonebook_id: &str,
        mut payload: PhoneBookCreateUpdateRequest,
    ) -> Result<PhoneBookCreateUpdateDeleteResponse, errors::HttpError> {
        payload.set_api_key(self.api_key);

        let response = self.client.patch(
            format!("phonebooks/{}", phonebook_id).as_str(),
            None,
            None,
            Some(payload),
        )?;

        let campaign_update_response =
            response_or_error_text_blocking!(response, PhoneBookCreateUpdateDeleteResponse);

        Ok(campaign_update_response)
    }

    /// Delete Phonebook.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     blocking::rest::termii, common::switch::campaign::PhoneBookCreateUpdateDeleteResponse,
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let phonebook_delete_response: PhoneBookCreateUpdateDeleteResponse = client
    ///     .switch
    ///     .campaign
    ///     .delete("f9c28de9-ab5a-4513-9c9f-338be8e1c390")
    ///     .unwrap();
    ///
    /// println!("{:?}", phonebook_delete_response);
    /// ```
    pub fn delete(
        &self,
        phonebook_id: &str,
    ) -> Result<PhoneBookCreateUpdateDeleteResponse, errors::HttpError> {
        let response =
            self.client
                .delete(format!("phonebooks/{}", phonebook_id).as_str(), None, None)?;

        let campaign_delete_response =
            response_or_error_text_blocking!(response, PhoneBookCreateUpdateDeleteResponse);

        Ok(campaign_delete_response)
    }
}

impl pagination::PaginatedResource for Campaign<'_> {
    type Item = PhoneBookItem;

    fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError> {
        Campaign::_get(self, page)
    }
}

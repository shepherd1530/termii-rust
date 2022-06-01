//! Send and manage campaigns sent to your organization's phonebook.
//!
//! Create, view & manage phonebooks using these APIs.
//!
//! Each phonebook can be identified by a unique ID, which makes it easier to edit or delete a phonebook.

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;

use crate::{
    async_impl::http::client,
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
    client: Arc<client::HttpClient>,
}

impl<'a> Campaign<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Campaign<'a> {
        Campaign { api_key, client }
    }

    pub(crate) async fn _get(&self, page: &str) -> Result<Vec<PhoneBookItem>, errors::HttpError> {
        let mut params = HashMap::new();
        params.insert("api_key", self.api_key);
        params.insert("page", page);

        let response = self.client.get("phonebooks", Some(params), None).await?;

        let campaign_item = response_or_error_text_async!(response, PhoneBookListResponse);

        Ok(campaign_item.data)
    }

    /// Fetch Phonebooks.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{async_impl::rest::termii, common::switch::campaign::PhoneBookItem};
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let phonebooks: Vec<PhoneBookItem> = client.switch.campaign.get(Some(1)).await.unwrap();
    ///
    /// println!("{:?}", phonebooks);
    /// ```
    /// ### The above code is limited by termii's pagination. You can get all your phonebooks with the **all** function like such
    ///
    /// ```rust
    /// let phonebooks: Vec<PhoneBookItem> = client.switch.campaign.all().await.unwrap();
    /// ```
    pub async fn get(&self, page: Option<&str>) -> Result<Vec<PhoneBookItem>, errors::HttpError> {
        let page = page.unwrap_or("1");
        let campaign_items = self._get(page).await?;
        Ok(campaign_items)
    }

    /// Fetch Phonebooks.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
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
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", phonebook_create_response);
    /// ```
    pub async fn create(
        &self,
        mut payload: PhoneBookCreateUpdateRequest,
    ) -> Result<PhoneBookCreateUpdateDeleteResponse, errors::HttpError> {
        payload.set_api_key(self.api_key);

        let response = self
            .client
            .post("phonebooks", None, None, Some(payload))
            .await?;

        let campaign_create_response =
            response_or_error_text_async!(response, PhoneBookCreateUpdateDeleteResponse);

        Ok(campaign_create_response)
    }

    /// Update Phonebook.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
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
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", phonebook_update_response);
    /// ```
    pub async fn update(
        &self,
        phonebook_id: &str,
        mut payload: PhoneBookCreateUpdateRequest,
    ) -> Result<PhoneBookCreateUpdateDeleteResponse, errors::HttpError> {
        payload.set_api_key(self.api_key);

        let response = self
            .client
            .patch(
                format!("phonebooks/{}", phonebook_id).as_str(),
                None,
                None,
                Some(payload),
            )
            .await?;

        let campaign_update_response =
            response_or_error_text_async!(response, PhoneBookCreateUpdateDeleteResponse);

        Ok(campaign_update_response)
    }

    /// Delete Phonebook.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii, common::switch::campaign::PhoneBookCreateUpdateDeleteResponse,
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let phonebook_delete_response: PhoneBookCreateUpdateDeleteResponse = client
    ///     .switch
    ///     .campaign
    ///     .delete("f9c28de9-ab5a-4513-9c9f-338be8e1c390")
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", phonebook_delete_response);
    /// ```
    pub async fn delete(
        &self,
        phonebook_id: &str,
    ) -> Result<PhoneBookCreateUpdateDeleteResponse, errors::HttpError> {
        let response = self
            .client
            .delete(format!("phonebooks/{}", phonebook_id).as_str(), None, None)
            .await?;

        let campaign_delete_response =
            response_or_error_text_async!(response, PhoneBookCreateUpdateDeleteResponse);

        Ok(campaign_delete_response)
    }
}

#[async_trait]
impl pagination::PaginatedResourceAsync for Campaign<'_> {
    type Item = PhoneBookItem;

    async fn _get(&self, page: &str) -> Result<Vec<Self::Item>, errors::HttpError> {
        Campaign::_get(self, page).await
    }
}

//! Request and Send template messageds across different messaging channels.

use std::sync::Arc;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        switch::templates::{TemplateItem, TemplatesRequest},
    },
};

#[derive(Debug)]
pub struct Templates<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> Templates<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Templates<'a> {
        Templates { api_key, client }
    }

    /// Set a template for your org's one time pin.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::switch::templates::{TemplatesData, TemplatesRequest},
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let templates_data =
    ///     TemplatesData::new("Termii", "325821".to_string(), "10 minutes".to_string());
    ///
    /// let templates_payload = TemplatesRequest::new(
    ///     "+234XXXXXXXXXX".to_string(),
    ///     "talert".to_string(),
    ///     "1493-csdn3-ns34w-sd3434-dfdf".to_string(),
    ///     templates_data,
    /// );
    ///
    /// let templates_response = client
    ///     .switch
    ///     .templates
    ///     .send(templates_payload)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", templates_response);
    /// ```
    pub async fn send<T>(
        &self,
        mut payload: TemplatesRequest,
    ) -> Result<Vec<TemplateItem>, errors::HttpError> {
        payload.set_api_key(self.api_key);

        let response = self
            .client
            .post("send/templates", None, None, Some(payload))
            .await?;

        let template = response_or_error_text_async!(response, Vec<TemplateItem>);

        Ok(template)
    }
}

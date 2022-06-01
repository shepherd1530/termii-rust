//! Send messages to your organization's customers using auto-generated messaging numbers.

use std::sync::Arc;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        switch::number::{NumberMessageRequest, NumberMessageResponse},
    },
};

#[derive(Debug)]
pub struct Number<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> Number<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Number<'a> {
        Number { api_key, client }
    }

    /// Send a message to a recipient using termii's auto generated number.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{async_impl::rest::termii, common::switch::number::NumberMessageRequest};
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let message_payload =
    ///     NumberMessageRequest::new("234XXXXXXXXXX".to_string(), "Your message".to_string());
    ///
    /// let message_response = client.switch.number.send(message_payload).await.unwrap();
    ///
    /// println!("{:?}", message_response);
    /// ```
    pub async fn send(
        &self,
        mut message: NumberMessageRequest,
    ) -> Result<NumberMessageResponse, errors::HttpError> {
        message.set_api_key(self.api_key);

        let response = self
            .client
            .post("sms/number/send", None, None, Some(message))
            .await?;

        let message_response = response_or_error_text_async!(response, NumberMessageResponse);

        Ok(message_response)
    }
}

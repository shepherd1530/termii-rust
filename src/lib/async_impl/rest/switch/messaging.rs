//! Send messages to customers across termii channels.

use std::sync::Arc;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        switch::messaging::{
            MessageBulkRequest, MessageBulkResponse, MessageRequest, MessageResponse,
        },
    },
};

#[derive(Debug)]
pub struct Messaging<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> Messaging<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Messaging<'a> {
        Messaging { api_key, client }
    }

    /// Send a message to a recipient.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::switch::messaging::{Channel, MessageRequest, MessageType},
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let message_payload = MessageRequest::new(
    ///     "234XXXXXXXXXX".to_string(),
    ///     "Your org sender id".to_string(),
    ///     "Your message".to_string(),
    ///     MessageType::Plain,
    ///     Channel::Generic,
    /// );
    ///
    /// let message_response = client.switch.messaging.send(message_payload).await.unwrap();
    ///
    /// println!("{:?}", message_response);
    /// ```
    pub async fn send(
        &self,
        mut message: MessageRequest,
    ) -> Result<MessageResponse, errors::HttpError> {
        message.set_api_key(self.api_key);

        let response = self
            .client
            .post("sms/send", None, None, Some(message))
            .await?;

        let message_response = response_or_error_text_async!(response, MessageResponse);

        Ok(message_response)
    }

    /// Send a message to multiple recipients.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::switch::messaging::{
    ///         Channel, MessageBulkRequest, MessageBulkResponse, MessageType,
    ///     },
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let message_bulk_payload = MessageBulkRequest::new(
    ///     vec!["234XXXXXXXXXX".to_string(), "234XXXXXXXXXX".to_string()],
    ///     "Your org sender id".to_string(),
    ///     "Your message".to_string(),
    ///     MessageType::Plain,
    ///     Channel::Generic,
    /// );
    ///
    /// let message_bulk_response = client
    ///     .switch
    ///     .messaging
    ///     .send_bulk(message_bulk_payload)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", message_bulk_response);
    /// ```
    pub async fn send_bulk(
        &self,
        mut message: MessageBulkRequest,
    ) -> Result<MessageBulkResponse, errors::HttpError> {
        message.set_api_key(self.api_key);

        let response = self
            .client
            .post("sms/send/bulk", None, None, Some(message))
            .await?;

        let message_response = response_or_error_text_async!(response, MessageBulkResponse);

        Ok(message_response)
    }
}

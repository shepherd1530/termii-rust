//! Allow business trigger one-time-passwords
//! across any available messaging channels.

use std::sync::Arc;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        token::request::{RequestTokenRequest, RequestTokenResponse},
    },
};

#[derive(Debug)]
pub struct RequestToken<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> RequestToken<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> RequestToken<'a> {
        RequestToken { api_key, client }
    }

    /// Send a one time token request.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::token::{
    ///         RequestTokenChannel, RequestTokenMessageType, RequestTokenPinType, RequestTokenRequest,
    ///     },
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let otp_request = RequestTokenRequest::new(
    ///     RequestTokenMessageType::ALPHANUMERIC,
    ///     String::from("234XXXXXXXXXX"),
    ///     String::from("Your org sender ID"),
    ///     RequestTokenChannel::Generic,
    ///     3 as u8,
    ///     50 as usize,
    ///     6 as u8,
    ///     String::from("< 1234 >"),
    ///     String::from("Your pin is < 1234 >"),
    ///     RequestTokenPinType::ALPHANUMERIC,
    /// );
    ///
    /// let response = client.token.request.send(otp_request).await.unwrap();
    ///
    /// println!("{:#?}", response);
    /// ```
    pub async fn send(
        &self,
        mut otp_payload: RequestTokenRequest,
    ) -> Result<RequestTokenResponse, errors::HttpError> {
        otp_payload.set_api_key(self.api_key);

        let response = self
            .client
            .post("sms/otp/send", None, None, Some(otp_payload))
            .await?;

        let otp_response = response_or_error_text_async!(response, RequestTokenResponse);

        Ok(otp_response)
    }
}

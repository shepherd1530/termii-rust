//! In-App token are numeric or alpha-numeric codes generated to authenticate
//! login requests and verify customer transactions.

use std::sync::Arc;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        token::in_app_token::{InAppTokenRequest, InAppTokenResponse},
    },
};

#[derive(Debug)]
pub struct InAppToken<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> InAppToken<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> InAppToken<'a> {
        InAppToken { api_key, client }
    }

    /// Fetch JSON In-App otp's.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{
    ///     async_impl::rest::termii,
    ///     common::token::{InAppTokenMessageType, InAppTokenRequest},
    /// };
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let in_app_token_request =
    ///     InAppTokenRequest::new("+234XXXXXXXXXX", InAppTokenMessageType::NUMERIC, 3, 300, 6);
    ///
    /// let in_app_token_response = client
    ///     .token
    ///     .in_app_token
    ///     .send(in_app_token_request)
    ///     .await
    ///     .unwrap();
    ///
    /// println!("{:?}", in_app_token_response);
    /// ```
    pub async fn send(
        &self,
        mut otp_payload: InAppTokenRequest,
    ) -> Result<InAppTokenResponse, errors::HttpError> {
        otp_payload.set_api_key(self.api_key);

        let response = self
            .client
            .post("sms/otp/generate", None, None, Some(otp_payload))
            .await?;

        let otp_response = response_or_error_text_async!(response, InAppTokenResponse);

        Ok(otp_response)
    }
}

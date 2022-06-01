//! Verify one-time passwords and return responses
//! based on the validity of the token.

use std::sync::Arc;

use crate::{
    async_impl::http::client,
    common::{
        errors,
        token::verify::{VerifyTokenRequest, VerifyTokenResponse},
    },
};

#[derive(Debug)]
pub struct VerifyToken<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
}

impl<'a> VerifyToken<'a> {
    pub(crate) fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> VerifyToken<'a> {
        VerifyToken { api_key, client }
    }

    /// Verify one-time passwords and return responses.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use termii_rust::{async_impl::rest::termii, common::token::VerifyTokenRequest};
    ///
    /// let client = termii::Termii::new("Your API key");
    ///
    /// let verify_otp_request = VerifyTokenRequest::new(
    ///     "c8dcd048-5e7f-4347-8c89-4470c3af0b".to_string(),
    ///     "195558".to_string(),
    /// );
    ///
    /// let response = client.token.verify.send(verify_otp_request).await.unwrap();
    ///
    /// println!("{:#?}", response);
    /// ```
    pub async fn send(
        &self,
        mut otp_payload: VerifyTokenRequest,
    ) -> Result<VerifyTokenResponse, errors::HttpError> {
        otp_payload.set_api_key(self.api_key);

        let response = self
            .client
            .post("sms/otp/verify", None, None, Some(otp_payload))
            .await?;

        let otp_response = response_or_error_text_async!(response, VerifyTokenResponse);

        Ok(otp_response)
    }
}

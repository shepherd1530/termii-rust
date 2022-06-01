use std::sync::Arc;

use crate::async_impl::{
    http::client,
    rest::token::{InAppToken, RequestToken, VerifyToken},
};

#[derive(Debug)]
pub struct Token<'a> {
    pub request_token: RequestToken<'a>,
    pub verify_token: VerifyToken<'a>,
    pub in_app_token: InAppToken<'a>,
}

impl<'a> Token<'a> {
    pub fn new(api_key: &str, client: Arc<client::HttpClient>) -> Token {
        let request_token = RequestToken::new(api_key, Arc::clone(&client));
        let verify_token = VerifyToken::new(api_key, Arc::clone(&client));
        let in_app_token = InAppToken::new(api_key, Arc::clone(&client));

        Token {
            request_token,
            verify_token,
            in_app_token,
        }
    }
}

use std::rc::Rc;

use crate::blocking::{
    http::client,
    rest::token::{InAppToken, RequestToken, VerifyToken},
};

#[derive(Debug)]
pub struct Token<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
    pub request_token: RequestToken<'a>,
    pub verify_token: VerifyToken<'a>,
    pub in_app_token: InAppToken<'a>,
}

impl<'a> Token<'a> {
    pub fn new(api_key: &str, client: Rc<client::HttpClient>) -> Token {
        let request_token = RequestToken::new(api_key, Rc::clone(&client));
        let verify_token = VerifyToken::new(api_key, Rc::clone(&client));
        let in_app_token = InAppToken::new(api_key, Rc::clone(&client));

        Token {
            api_key,
            client,
            request_token,
            verify_token,
            in_app_token,
        }
    }
}

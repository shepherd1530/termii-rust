use std::sync::Arc;

use rest::{insights, switch, token};

use crate::async_impl::{http::client, rest};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Termii<'a> {
    pub token: token::Token<'a>,
    pub insights: insights::Insights<'a>,
    pub switch: switch::Switch<'a>,
}

impl<'a> Termii<'a> {
    pub fn new(api_key: &str) -> Termii {
        let http_client = Arc::new(
            client::HttpClient::new(20).expect("Can not create new instance of the http client."),
        );

        let token = token::Token::new(api_key, Arc::clone(&http_client));
        let switch = switch::Switch::new(api_key, Arc::clone(&http_client));
        let insights = insights::Insights::new(api_key, Arc::clone(&http_client));

        println!("{}", Arc::strong_count(&http_client));

        let termii = Termii {
            insights: insights, 
            token: token,
            switch: switch,
        };

        termii
    }
}

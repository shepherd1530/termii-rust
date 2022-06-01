use std::rc::Rc;

use rest::{insights, switch, token};

use crate::blocking::{http::client, rest};

#[derive(Debug)]
pub struct Termii<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
    pub token: token::Token<'a>,
    pub insights: insights::Insights<'a>,
    pub switch: switch::Switch<'a>,
}

impl<'a> Termii<'a> {
    pub fn new(api_key: &str) -> Termii {
        let http_client = Rc::new(
            client::HttpClient::new(20).expect("Can not create new instance of the http client."),
        );

        let token = token::Token::new(api_key, Rc::clone(&http_client));
        let switch = switch::Switch::new(api_key, Rc::clone(&http_client));
        let insights = insights::Insights::new(api_key, Rc::clone(&http_client));

        println!("{}", Rc::strong_count(&http_client));

        let termii = Termii {
            insights: insights,
            api_key: api_key,
            client: http_client,
            token: token,
            switch: switch,
        };

        termii
    }
}

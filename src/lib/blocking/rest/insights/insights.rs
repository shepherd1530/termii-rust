use std::rc::Rc;

use crate::{
    blocking::{
        http::client,
        rest::insights::{Balance, History, Search, Status},
    },
    common::errors,
};

#[derive(Debug)]
pub struct Insights<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
    pub balance: Balance<'a>,
    pub history: History<'a>,
    pub search: Search<'a>,
    pub status: Status<'a>,
}

impl<'a> Insights<'a> {
    pub fn new(api_key: &'a str, client: Rc<client::HttpClient>) -> Insights<'a> {
        let balance = Balance::new(&api_key, Rc::clone(&client));
        let history = History::new(&api_key, Rc::clone(&client));
        let search = Search::new(&api_key, Rc::clone(&client));
        let status = Status::new(&api_key, Rc::clone(&client));

        Insights {
            api_key,
            client,
            balance,
            history,
            search,
            status,
        }
    }
}

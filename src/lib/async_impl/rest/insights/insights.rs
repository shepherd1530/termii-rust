use std::sync::Arc;

use crate::{
    async_impl::{
        http::client,
        rest::insights::{Balance, History, Search, Status},
    },
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Insights<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
    pub balance: Balance<'a>,
    pub history: History<'a>,
    pub search: Search<'a>,
    pub status: Status<'a>,
}

impl<'a> Insights<'a> {
    pub fn new(api_key: &'a str, client: Arc<client::HttpClient>) -> Insights<'a> {
        let balance = Balance::new(&api_key, Arc::clone(&client));
        let history = History::new(&api_key, Arc::clone(&client));
        let search = Search::new(&api_key, Arc::clone(&client));
        let status = Status::new(&api_key, Arc::clone(&client));

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

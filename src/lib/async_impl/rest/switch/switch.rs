use std::sync::Arc;

use crate::async_impl::{
    http::client,
    rest::switch::{Campaign, Messaging, Number, SenderID, Templates},
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Switch<'a> {
    api_key: &'a str,
    client: Arc<client::HttpClient>,
    pub templates: Templates<'a>,
    pub sender_id: SenderID<'a>,
    pub campaign: Campaign<'a>,
    pub number: Number<'a>,
    pub messaging: Messaging<'a>,
}

impl<'a> Switch<'a> {
    pub fn new(api_key: &str, client: Arc<client::HttpClient>) -> Switch {
        let templates = Templates::new(&api_key, Arc::clone(&client));
        let sender_id = SenderID::new(&api_key, Arc::clone(&client));
        let campaign = Campaign::new(&api_key, Arc::clone(&client));
        let number = Number::new(&api_key, Arc::clone(&client));
        let messaging = Messaging::new(&api_key, Arc::clone(&client));

        Switch {
            api_key,
            client,
            templates,
            sender_id,
            campaign,
            number,
            messaging,
        }
    }
}

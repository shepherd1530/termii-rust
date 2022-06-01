use std::rc::Rc;

use crate::blocking::{
    http::client,
    rest::switch::{Campaign, Messaging, Number, SenderID, Templates},
};

#[derive(Debug)]
pub struct Switch<'a> {
    api_key: &'a str,
    client: Rc<client::HttpClient>,
    pub templates: Templates<'a>,
    pub sender_id: SenderID<'a>,
    pub campaign: Campaign<'a>,
    pub number: Number<'a>,
    pub messaging: Messaging<'a>,
}

impl<'a> Switch<'a> {
    pub fn new(api_key: &str, client: Rc<client::HttpClient>) -> Switch {
        let templates = Templates::new(&api_key, Rc::clone(&client));
        let sender_id = SenderID::new(&api_key, Rc::clone(&client));
        let campaign = Campaign::new(&api_key, Rc::clone(&client));
        let number = Number::new(&api_key, Rc::clone(&client));
        let messaging = Messaging::new(&api_key, Rc::clone(&client));

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

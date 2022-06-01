//! Send messages globally.

//! # Examples
//!
//!
//! Fetch Phonebooks.
//!
//! ```rust
//! use termii_rust::{blocking::rest::termii, common::switch::campaign::PhoneBookItem};
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let phonebooks: Vec<PhoneBookItem> = client.switch.campaign.get(Some(1)).unwrap();
//!
//! println!("{:?}", phonebooks);
//! ```
//! ### The above code is limited by termii's pagination. You can get all your phonebooks with the **all** function like such
//!
//! ```rust
//! let phonebooks: Vec<PhoneBookItem> = client.switch.campaign.all().unwrap();
//! ```
//!
//!
//! ## Send a message to a recipient.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::switch::messaging::{Channel, MessageRequest, MessageType},
//! };
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let message_payload = MessageRequest::new(
//!     "234XXXXXXXXXX".to_string(),
//!     "Your org sender id".to_string(),
//!     "Your message".to_string(),
//!     MessageType::Plain,
//!     Channel::Generic,
//! );
//!
//! let message_response = client.switch.messaging.send(message_payload).unwrap();
//!
//! println!("{:?}", message_response);
//! ```
//!
//!
//! ## Send a message to a recipient using termii's auto generated number.
//!
//! ```rust
//! use termii_rust::{blocking::rest::termii, common::switch::number::NumberMessageRequest};
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let message_payload =
//!     NumberMessageRequest::new("234XXXXXXXXXX".to_string(), "Your message".to_string());
//!
//! let message_response = client.switch.number.send(message_payload).unwrap();
//!
//! println!("{:?}", message_response);
//! ```
//!
//!
//! ## Fetch your organization's sender ID's.
//!
//! ```rust
//! use termii_rust::blocking::rest::termii;
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let sender_id = client.switch.sender_id.get(Some("1")).unwrap();
//!
//! println!("{:?}", sender_id);
//! ```
//! ### The above code is limited by termii's pagination. You can get all your sender ID's with the **all** function like such
//!
//! ```rust
//! let sender_ids = client.switch.sender_id.all().unwrap();
//! ```
//!
//!
//! ## Set a template for your org's one time pin.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::switch::templates::{TemplatesData, TemplatesRequest},
//! };
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let templates_data =
//!     TemplatesData::new("Termii", "325821".to_string(), "10 minutes".to_string());
//!
//! let templates_payload = TemplatesRequest::new(
//!     "+234XXXXXXXXXX".to_string(),
//!     "talert".to_string(),
//!     "1493-csdn3-ns34w-sd3434-dfdf".to_string(),
//!     templates_data,
//! );
//!
//! let templates_response = client.switch.templates.send(templates_payload).unwrap();
//!
//! println!("{:?}", templates_response);
//! ```
pub mod switch;
pub use switch::*;

pub mod templates;
pub use templates::*;

pub mod sender_id;
pub use sender_id::*;

pub mod campaign;
pub use campaign::*;

pub mod number;
pub use number::*;

pub mod messaging;
pub use messaging::*;

//! A blocking messaging client API.
//!
//! This module provides a blocking interface to the termii API. This means all methods will block
//! the current thread until the request is completed. Using this module within an async runtime
//! will result in a panic. If this is not what you want, use the [`async_impl`][crate::async_impl] module.
//!
//! ## Sending a quick message
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::switch::messaging::{Channel, MessageRequest, MessageType},
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let _message = MessageRequest::new(
//!     "234XXXXXXXXXX".to_string(),
//!     "FromYourOrg".to_string(),
//!     "Hello from Rust Termii. 😎".to_string(),
//!     MessageType::Plain,
//!     Channel::Dnd,
//! );
//!
//! let message = client.switch.messaging.send(_message).unwrap();
//!
//! println!("{:?}", message);
//! ```
//!
//! ## Sending a one time token
//!
//! We can use the [`token`](crate::blocking::rest::token) module of the Token api to send a one time token.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::token::request::{
//!         RequestTokenChannel, RequestTokenMessageType, RequestTokenPinType, RequestTokenRequest,
//!     },
//! };
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let payload = RequestTokenRequest::new(
//!     RequestTokenMessageType::ALPHANUMERIC,
//!     String::from("234XXXXXXXXXX"),
//!     String::from("FromYourOrg"),
//!     RequestTokenChannel::Generic,
//!     3 as u8,
//!     50 as usize,
//!     6 as u8,
//!     String::from("< 1234 >"),
//!     String::from("Your pin is < 1234 >"),
//!     RequestTokenPinType::ALPHANUMERIC,
//! );
//!
//! let response = client.token.request_token.send(otp_payload).unwrap();
//!
//! println!("{:#?}", response);
//! ```

pub mod http;
pub mod rest;

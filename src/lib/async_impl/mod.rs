//! An asynchronous messaging client API.
//!
//! This module provides an async interface to the termii API. Yow will need a runtime to use this module.
//! If you want to use this module within a blocking runtime, use the [`blocking::rest`][crate::blocking::rest] module.
//!
//! ## Sending a quick message
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
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
//! let message = client.switch.messaging.send(_message).await.unwrap();
//!
//! println!("{:?}", message);
//! ```
//!
//! ## Sending a one time token
//!
//! We can use the [`token`](crate::async_impl::rest::token) module of the Token api to send a one time token.
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
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
//! let response = client.token.request_token.send(otp_payload).await.unwrap();
//!
//! println!("{:#?}", response);
//! ```

pub mod http;
pub mod rest;

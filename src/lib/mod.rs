//! # Termii Rust
//!
//! The `termii_rust` crate provides a Rust interface to the Termii messaging platform. It contains handlers for termii's modules like
//!
//! - Switch API: Including the Sender ID, Messaging, Number, Templates and Campaign API's
//! - Token API: Including the Send Token, Verify Token and In-App Token API's.
//! - Insights API: Including the Balance, Search, Status and History API's.
//!
//! The crate provides both blocking and async interfaces to the Termii API. The blocking interface is behind the blocking module likewise
//! the async interface is behind the async_impl module but is also exported and can be used directly from the root of the crate.
//!
//! ## Sending a quick message
//!
//! We can use the [`messaging`](async_impl::rest::switch::messaging) module of the Switch api to send messages.
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
//!     common::switch::messaging::{Channel, MessageRequest, MessageType},
//! };
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
//! let message = client.switch.messaging.send(_message).await;
//!
//! println!("{:?}", message);
//! ```
//!
//! ## Sending a one time token
//!
//! We can use the [`token`](async_impl::rest::token) module of the Token api to send a one time token.
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
//! let response = client.token.request_token.send(otp_payload).await;
//!
//! println!("{:#?}", response);
//! ```
//!
//! ## Gets your messaging history.
//!
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
//!     common::{insights::history::HistoryItem, pagination::PaginatedResourceAsync},
//! };
//!
//! let client = termii::Termii::new("Your API key");
//! 
//! let all_history = client.insights.history.all().await;
//!
//! println!("{:?}", all_history);
//! ```
//!
//! ## Optional Features
//!
//! The crate provides an optional [`blocking`][blocking] module which provides a blocking interface to the Termii API.

#[macro_use]
pub mod macros;

pub mod common;

pub mod async_impl;
pub use async_impl::*;

#[cfg(feature = "blocking")]
pub mod blocking;

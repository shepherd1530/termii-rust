//! Secure transactions for users.

//! # Examples
//!
//!
//! ## Fetch JSON In-App otp's.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::token::{InAppTokenMessageType, InAppTokenRequest},
//! };
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let in_app_token_request =
//!     InAppTokenRequest::new("+234XXXXXXXXXX", InAppTokenMessageType::NUMERIC, 3, 300, 6);
//!
//! let in_app_token_response = client
//!     .token
//!     .in_app_token
//!     .send(in_app_token_request)
//!     .unwrap();
//!
//! println!("{:?}", in_app_token_response);
//! ```
//!
//!
//! ## Send a one time token request.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::token::{
//!         RequestTokenChannel, RequestTokenMessageType, RequestTokenPinType, RequestTokenRequest,
//!     },
//! };
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let otp_request = RequestTokenRequest::new(
//!     RequestTokenMessageType::ALPHANUMERIC,
//!     String::from("234XXXXXXXXXX"),
//!     String::from("Your org sender ID"),
//!     RequestTokenChannel::Generic,
//!     3 as u8,
//!     50 as usize,
//!     6 as u8,
//!     String::from("< 1234 >"),
//!     String::from("Your pin is < 1234 >"),
//!     RequestTokenPinType::ALPHANUMERIC,
//! );
//!
//! let response = client.token.request.send(otp_request).unwrap();
//!
//! println!("{:#?}", response);
//! ```
//!
//!
//! ## Verify one-time passwords and return responses.
//!
//!
//! ```rust
//! use termii_rust::{blocking::rest::termii, common::token::VerifyTokenRequest};
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let verify_otp_request = VerifyTokenRequest::new(
//!     "c8dcd048-5e7f-4347-8c89-4470c3af0b".to_string(),
//!     "195558".to_string(),
//! );
//!
//! let response = client.token.verify.send(verify_otp_request).unwrap();
//!
//! println!("{:#?}", response);
//! ```

pub mod token;
pub use token::*;

pub mod request;
pub use request::*;

pub mod verify;
pub use verify::*;

pub mod in_app_token;
pub use in_app_token::*;

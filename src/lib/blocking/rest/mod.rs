//! Provides blocking REST clients for the Termii API.
//!
//! This module provides all of the supported Termii features which includes
//!
//! - [`switch`](crate::blocking::rest::switch) : Send messages, bulk messages, fetch/request sender ids, CRUD phonebooks.
//! - [`token`](crate::blocking::rest::token) : Send and verify OTP messages for quick logins.
//! - [`insights`](crate::blocking::rest::insights) :  Retrieve balance, search phone number, retrieve phone number status and view message history.

pub mod insights;
pub mod switch;
pub mod termii;
pub mod token;

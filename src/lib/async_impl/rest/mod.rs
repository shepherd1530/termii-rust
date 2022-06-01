//! Provides asynchronous REST clients for the Termii API.
//!
//! This module provides all of the supported Termii features which includes
//!
//! - [`switch`](crate::async_impl::rest::switch) : Send messages, bulk messages, fetch/request sender ids, CRUD phonebooks.
//! - [`token`](crate::async_impl::rest::token) : Send and verify OTP messages for quick logins.
//! - [`insights`](crate::async_impl::rest::insights) :  Retrieve balance, search phone number, retrieve phone number status and view message history.

pub mod insights;
pub mod switch;
pub mod termii;
pub mod token;

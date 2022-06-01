//! Retrieve balance, search phone number, retrieve phone number status and view message history.

//! # Examples
//!
//! ## Get your account balance.
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
//!     common::insights::balance::BalanceItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let balance = client.insights.balance.get().await.unwrap();
//!
//! println!("{:?}", balance);
//! ```
//!
//!
//! ## Get your messaging history.
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
//!     common::insights::history::HistoryItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let history:HistoryItem = client.insights.history.get().await.unwrap();
//!
//! println!("{:?}", history);
//! ```
//!
//! ### The above code is limited by termii's pagination. You can get all your messaging history with the **all** function like such
//! ```rust
//! let history = client.insights.history.all().await.unwrap();
//! ```
//!
//!
//! ## Verify a phone number.
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
//!     common::insights::search::SearchItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let search:SearchItem = client.insights.search.get("234XXXXXXXXXX").await.unwrap();
//!
//! println!("{:?}", search);
//! ```
//!
//!
//! ## Detects fake or ported numbers.
//
//!
//! ```rust
//! use termii_rust::{
//!     async_impl::rest::termii,
//!     common::insights::status::StatusItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let status:StatusItem = client.insights.status.get("234XXXXXXXXXX", "NG").await.unwrap();
//!
//! println!("{:?}", status);
//! ```

pub mod insights;
pub use insights::*;

pub mod balance;
pub use balance::*;

pub mod history;
pub use history::*;

pub mod search;
pub use search::*;

pub mod status;
pub use status::*;

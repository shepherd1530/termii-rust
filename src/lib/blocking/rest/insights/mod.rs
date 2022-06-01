//! Get insights about your organization's activities.

//! # Examples
//!
//! ## Gets your account balance.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::insights::balance::BalanceItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let balance:BalanceItem = client.insights.balance.get().unwrap();
//!
//! println!("{:?}", balance);
//! ```
//!
//!
//! ## Gets your messaging history.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::insights::history::HistoryItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let history:HistoryItem = client.insights.history.get().unwrap();
//!
//! println!("{:?}", history);
//! ```
//!
//! ### The above code is limited by termii's pagination. You can get all your messaging history with the **all** function like such
//! ```rust
//! let history = client.insights.history.all().unwrap();
//! ```
//!
//!
//! Verify a phone number.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::insights::search::SearchItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let search:SearchItem = client.insights.search.get("234XXXXXXXXXX").unwrap();
//!
//! println!("{:?}", search);
//! ```
//!
//!
//! ## Detects fake or ported numbers.
//!
//! ```rust
//! use termii_rust::{
//!     blocking::rest::termii,
//!     common::insights::status::StatusItem,
//! }
//!
//! let client = termii::Termii::new("Your API key");
//!
//! let status:StatusItem = client.insights.status.get("234XXXXXXXXXX", "NG").unwrap();
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

#![deny(missing_docs)]
//! # Crypto.com Chain Client
//!
//! This crate exposes following functionalities for interacting with Crypto.com Chain:
//! - Wallet creation
//! - Address generation
//! - Balance tracking
//! - Transaction creation and signing
//!
//! ## A note on features
//!
//! This crate has features! Here's a list of features you can enable:
//! - Persistent storage implementation using [`Sled`](https://crates.io/crates/sled)
//!   - This feature implements [`Storage`](crate::Storage) trait using `Sled` embedded database.
//!   - Enable with `"sled"` feature flag.
//!   - This feature is enabled by default.
//!
pub mod error;
pub mod key;
pub mod storage;

#[doc(inline)]
pub use error::{Error, ErrorKind, Result};
#[doc(inline)]
pub use storage::{SecureStorage, Storage};

use secp256k1::{All, Secp256k1};

thread_local! { pub(crate) static SECP: Secp256k1<All> = Secp256k1::new(); }

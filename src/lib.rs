// Magical Bitcoin Library
// Written in 2020 by
//     Alekos Filini <alekos.filini@gmail.com>
//
// Copyright (c) 2020 Magical Bitcoin
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// rustdoc will warn if there are missing docs
#![warn(missing_docs)]
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]
// only enables the nightly `external_doc` feature when
// `test-md-docs` is enabled
#![cfg_attr(feature = "test-md-docs", feature(external_doc))]

//! A modern, lightweight, descriptor-based wallet library written in Rust.
//!
//! # About
//!
//! The BDK library aims to be the core building block for Bitcoin wallets of any kind.
//!
//! * It uses [Miniscript](https://github.com/rust-bitcoin/rust-miniscript) to support descriptors with generalized conditions. This exact same library can be used to build
//!   single-sig wallets, multisigs, timelocked contracts and more.
//! * It supports multiple blockchain backends and databases, allowing developers to choose exactly what's right for their projects.
//! * It is built to be cross-platform: the core logic works on desktop, mobile, and even WebAssembly.
//! * It is very easy to extend: developers can implement customized logic for blockchain backends, databases, signers, coin selection, and more, without having to fork and modify this library.
//!
//! # A Tour of BDK
//!
//! BDK consists of a number of modules that provide a range of functionality
//! essential for implementing descriptor based Bitcoin wallet applications in Rust. In this
//! section, we will take a brief tour of BDK, summarizing the major APIs and
//! their uses.
//!
//! The easiest way to get started is to add bdk to your dependencies with the default features.
//! The default features include a simple key-value database ([`sled`](sled)) to cache
//! blockchain data and an [electrum](https://docs.rs/electrum-client/) blockchain client to
//! interact with the bitcoin P2P network.
//!
//! ```toml
//! bdk = "0.2.0"
//! ```
//!
//! ## Sync the balance of a descriptor
//!
//! ### Example
//! ```ignore
//! use bdk::Wallet;
//! use bdk::database::MemoryDatabase;
//! use bdk::blockchain::{noop_progress, ElectrumBlockchain};
//!
//! use bdk::electrum_client::Client;
//!
//! fn main() -> Result<(), bdk::Error> {
//!     let client = Client::new("ssl://electrum.blockstream.info:60002")?;
//!     let wallet = Wallet::new(
//!         "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
//!         Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
//!         bitcoin::Network::Testnet,
//!         MemoryDatabase::default(),
//!         ElectrumBlockchain::from(client)
//!     )?;
//!
//!     wallet.sync(noop_progress(), None)?;
//!
//!     println!("Descriptor balance: {} SAT", wallet.get_balance()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Generate a few addresses
//!
//! ### Example
//! ```
//! use bdk::{Wallet, OfflineWallet};
//! use bdk::database::MemoryDatabase;
//!
//! fn main() -> Result<(), bdk::Error> {
//!     let wallet: OfflineWallet<_> = Wallet::new_offline(
//!         "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
//!         Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
//!         bitcoin::Network::Testnet,
//!         MemoryDatabase::default(),
//!     )?;
//!
//!     println!("Address #0: {}", wallet.get_new_address()?);
//!     println!("Address #1: {}", wallet.get_new_address()?);
//!     println!("Address #2: {}", wallet.get_new_address()?);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Create a transaction
//!
//! ### Example
//! ```ignore
//! use base64::decode;
//! use bdk::{FeeRate, TxBuilder, Wallet};
//! use bdk::database::MemoryDatabase;
//! use bdk::blockchain::{noop_progress, ElectrumBlockchain};
//!
//! use bdk::electrum_client::Client;
//!
//! use bitcoin::consensus::serialize;
//!
//! fn main() -> Result<(), bdk::Error> {
//!     let client = Client::new("ssl://electrum.blockstream.info:60002")?;
//!     let wallet = Wallet::new(
//!         "wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)",
//!         Some("wpkh([c258d2e4/84h/1h/0h]tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/1/*)"),
//!         bitcoin::Network::Testnet,
//!         MemoryDatabase::default(),
//!         ElectrumBlockchain::from(client)
//!     )?;
//!
//!     wallet.sync(noop_progress(), None)?;
//!
//!     let send_to = wallet.get_new_address()?;
//!     let (psbt, details) = wallet.create_tx(
//!         TxBuilder::with_recipients(vec![(send_to.script_pubkey(), 50_000)])
//!             .enable_rbf()
//!             .do_not_spend_change()
//!             .fee_rate(FeeRate::from_sat_per_vb(5.0))
//!     )?;
//!
//!     println!("Transaction details: {:#?}", details);
//!     println!("Unsigned PSBT: {}", base64::encode(&serialize(&psbt)));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Sign a transaction
//!
//! ### Example
//! ```ignore
//! use base64::decode;
//! use bdk::{Wallet, OfflineWallet};
//! use bdk::database::MemoryDatabase;
//!
//! use bitcoin::consensus::deserialize;
//!
//! fn main() -> Result<(), bdk::Error> {
//!     let wallet: OfflineWallet<_> = Wallet::new_offline(
//!         "wpkh([c258d2e4/84h/1h/0h]tprv8griRPhA7342zfRyB6CqeKF8CJDXYu5pgnj1cjL1u2ngKcJha5jjTRimG82ABzJQ4MQe71CV54xfn25BbhCNfEGGJZnxvCDQCd6JkbvxW6h/0/*)",
//!         Some("wpkh([c258d2e4/84h/1h/0h]tprv8griRPhA7342zfRyB6CqeKF8CJDXYu5pgnj1cjL1u2ngKcJha5jjTRimG82ABzJQ4MQe71CV54xfn25BbhCNfEGGJZnxvCDQCd6JkbvxW6h/1/*)"),
//!         bitcoin::Network::Testnet,
//!         MemoryDatabase::default(),
//!     )?;
//!
//!     let psbt = "...";
//!     let psbt = deserialize(&base64::decode(psbt).unwrap())?;
//!
//!     let (signed_psbt, finalized) = wallet.sign(psbt, None)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! # Feature flags
//!
//! BDK uses a set of [feature flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section)
//! to reduce the amount of compiled code by allowing projects to only enable the features they need.
//! By default, BDK enables two internal features, `key-value-db` and `electrum`.
//!
//! If you are new to BDK we recommended that you use the default features which will enable
//! basic descriptor wallet functionality. More advanced users can disable the `default` features
//! (`--no-default-features`) and build the BDK library with only the features you need.

//! Below is a list of the available feature flags and the additional functionality they provide.
//!
//! * `all-keys`: all features for working with bitcoin keys
//! * `async-interface`: async functions in bdk traits
//! * `cli-utils`: utilities for creating a command line interface wallet
//! * `keys-bip39`: [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) mnemonic codes for generating deterministic keys
//!
//! ## Internal features
//!
//! These features do not expose any new API, but influence internal implementation aspects of
//! BDK.
//!
//! * `compact_filters`: [`compact_filters`](crate::blockchain::compact_filters) client protocol for interacting with the bitcoin P2P network
//! * `electrum`: [`electrum`](crate::blockchain::electrum) client protocol for interacting with electrum servers
//! * `esplora`: [`esplora`](crate::blockchain::esplora) client protocol for interacting with blockstream [electrs](https://github.com/Blockstream/electrs) servers
//! * `key-value-db`: key value [`database`](crate::database) based on [`sled`](crate::sled) for caching blockchain data

pub extern crate bitcoin;
extern crate log;
pub extern crate miniscript;
extern crate serde;
#[macro_use]
extern crate serde_json;

#[cfg(feature = "keys-bip39")]
extern crate bip39;

#[cfg(any(target_arch = "wasm32", feature = "async-interface"))]
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate bdk_macros;

#[cfg(feature = "compact_filters")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "electrum")]
pub extern crate electrum_client;

#[cfg(feature = "esplora")]
pub extern crate reqwest;

#[cfg(feature = "key-value-db")]
pub extern crate sled;

#[cfg(feature = "cli-utils")]
pub mod cli;

#[allow(unused_imports)]
#[cfg(test)]
#[macro_use]
extern crate testutils;
#[allow(unused_imports)]
#[cfg(test)]
#[macro_use]
extern crate testutils_macros;
#[allow(unused_imports)]
#[cfg(test)]
#[macro_use]
extern crate serial_test;

#[macro_use]
pub(crate) mod error;
pub mod blockchain;
pub mod database;
pub mod descriptor;
#[cfg(feature = "test-md-docs")]
mod doctest;
pub mod keys;
pub(crate) mod psbt;
pub(crate) mod types;
pub mod wallet;

pub use descriptor::template;
pub use descriptor::HDKeyPaths;
pub use error::Error;
pub use types::*;
pub use wallet::address_validator;
pub use wallet::signer;
pub use wallet::tx_builder::TxBuilder;
pub use wallet::{OfflineWallet, Wallet};

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Project
#### Added
- Add CONTRIBUTING.md
- Add a Discord badge to the README
- Add code coverage github actions workflow
- Add scheduled audit check in CI
- Add CHANGELOG.md

#### Changed
- Rename the library to `bdk`
- Rename `ScriptType` to `KeychainKind`
- Prettify README examples on github
- Change CI to github actions
- Bump rust-bitcoin to 0.25, fix Cargo dependencies
- Enable clippy for stable and tests by default
- Switch to "mainline" rust-miniscript
- Generate a different cache key for every CI job
- Fix to at least bitcoin ^0.25.2

#### Fixed
- Fix or ignore clippy warnings for all optional features except compact_filters
- Pin cc version because last breaks rocksdb build

### Blockchain
#### Added
- Add a trait to create `Blockchain`s from a configuration
- Add an `AnyBlockchain` enum to allow switching at runtime
- Document `AnyBlockchain` and `ConfigurableBlockchain`
- Use our Instant struct to be compatible with wasm
- Make esplora call in parallel
- Allow to set concurrency in Esplora config and optionally pass it in repl

#### Fixed
- Fix receiving a coinbase using Electrum/Esplora
- Use proper type for EsploraHeader, make conversion to BlockHeader infallible
- Eagerly unwrap height option, save one collect

#### Changed
- Simplify the architecture of blockchain traits
- Improve sync
- Remove unused varaint HeaderParseFail

### CLI
#### Added
- Conditionally remove cli args according to enabled feature

#### Changed
- Add max_addresses param in sync
- Split the internal and external policy paths

### Database
#### Added
- Add `AnyDatabase` and `ConfigurableDatabase` traits

### Descriptor
#### Added
- Add a macro to write descriptors from code
- Add descriptor templates, add `DerivableKey`
- Add ToWalletDescriptor trait tests
- Add support for `sortedmulti` in `descriptor!`
- Add ExtractPolicy trait tests
- Add get_checksum tests, cleanup tests
- Add descriptor macro tests

#### Changes
- Improve the descriptor macro, add traits for key and descriptor types

#### Fixes
- Fix the recovery of a descriptor given a PSBT

### Keys
#### Added
- Add BIP39 support
- Take `ScriptContext` into account when converting keys
- Add a way to restrict the networks in which keys are valid
- Add a trait for keys that can be generated
- Fix entropy generation
- Less convoluted entropy generation
- Re-export tiny-bip39
- Implement `GeneratableKey` trait for `bitcoin::PrivateKey`
- Implement `ToDescriptorKey` trait for `GeneratedKey`
- Add a shortcut to generate keys with the default options

#### Fixed
- Fix all-keys and cli-utils tests

### Wallet
#### Added
- Allow to define static fees for transactions Fixes #137
- Merging two match expressions for fee calculation
- Incorporate RBF rules into utxo selection function
- Add Branch and Bound coin selection
- Add tests for BranchAndBoundCoinSelection::coin_select
- Add tests for BranchAndBoundCoinSelection::bnb
- Add tests for BranchAndBoundCoinSelection::single_random_draw
- Add test that shwpkh populates witness_utxo
- Add witness and redeem scripts to PSBT outputs
- Add an option to include `PSBT_GLOBAL_XPUB`s in PSBTs
- Eagerly finalize inputs

#### Changed
- Use collect to avoid iter unwrapping Options
- Make coin_select take may/must use utxo lists
- Improve `CoinSelectionAlgorithm`
- Refactor `Wallet::bump_fee()`
- Default to SIGHASH_ALL if not specified
- Replace ChangeSpendPolicy::filter_utxos with a predicate
- Make 'unspendable' into a HashSet
- Stop implicitly enforcing manaul selection by .add_utxo
- Rename DumbCS to LargestFirstCoinSelection
- Rename must_use_utxos to required_utxos
- Rename may_use_utxos to optional_uxtos
- Rename get_must_may_use_utxos to preselect_utxos
- Remove redundant Box around address validators
- Remove redundant Box around signers
- Make Signer and AddressValidator Send and Sync
- Split `send_all` into `set_single_recipient` and `drain_wallet`
- Use TXIN_DEFAULT_WEIGHT constant in coin selection
- Replace `must_use` with `required` in coin selection
- Take both spending policies into account in create_tx
- Check last derivation in cache to avoid recomputation
- Use the branch-and-bound cs by default
- Make coin_select return UTXOs instead of TxIns
- Build output lookup inside complete transaction
- Don't wrap SignersContainer arguments in Arc
- More consistent references with 'signers' variables

#### Fixed
- Fix signing for `ShWpkh` inputs
- Fix the recovery of a descriptor given a PSBT

### Examples
#### Added
- Support esplora blockchain source in repl

#### Changed
- Revert back the REPL example to use Electrum
- Remove the `magic` alias for `repl`
- Require esplora feature for repl example

#### Security
- Use dirs-next instead of dirs since the latter is unmantained

## [0.1.0-beta.1] - 2020-09-08

### Blockchain
#### Added
- Lightweight Electrum client with SSL/SOCKS5 support
- Add a generalized "Blockchain" interface
- Add Error::OfflineClient
- Add the Esplora backend
- Use async I/O in the various blockchain impls
- Compact Filters blockchain implementation
- Add support for Tor
- Impl OnlineBlockchain for types wrapped in Arc

### Database
#### Added
- Add a generalized database trait and a Sled-based implementation
- Add an in-memory database

### Descriptor
#### Added
- Wrap Miniscript descriptors to support xpubs
- Policy and contribution
- Transform a descriptor into its "public" version
- Use `miniscript::DescriptorPublicKey`

### Macros
#### Added
- Add a feature to enable the async interface on non-wasm32 platforms

### Wallet
#### Added
- Wallet logic
- Add `assume_height_reached` in PSBTSatisfier
- Add an option to change the assumed current height
- Specify the policy branch with a map
- Add a few commands to handle psbts
- Add hd_keypaths to outputs
- Add a `TxBuilder` struct to simplify `create_tx()`'s interface
- Abstract coin selection in a separate trait
- Refill the address pool whenever necessary
- Implement the wallet import/export format from FullyNoded
- Add a type convert fee units, add `Wallet::estimate_fee()`
- TxOrdering, shuffle/bip69 support
- Add RBF and custom versions in TxBuilder
- Allow limiting the use of internal utxos in TxBuilder
- Add `force_non_witness_utxo()` to TxBuilder
- RBF and add a few tests
- Add AddressValidators
- Add explicit ordering for the signers
- Support signing the whole tx instead of individual inputs
- Create a PSBT signer from an ExtendedDescriptor

### Examples
#### Added
- Add REPL broadcast command
- Add a miniscript compiler CLI
- Expose list_transactions() in the REPL
- Use `MemoryDatabase` in the compiler example
- Make the REPL return JSON

[unreleased]: https://github.com/bitcoindevkit/bdk/compare/0.1.0-beta.1...HEAD
[0.1.0-beta.1]: https://github.com/bitcoindevkit/bdk/compare/96c87ea5...0.1.0-beta.1

//! GrammarBot crate - check your sentence for grammar.
//!
//! This crate uses [official API](https://www.grammarbot.io/) of
//! [the GrammarBot service](http://grammarbot.io/).
//!
//! The service has rate limits:
//! As of this moment:
//! - 250 requests/day (~7500/mo) with an API key,
//! - 100 per day per IP address (~3000/mo) without an API key.
//!
//! # Usage
//!
#![cfg_attr(
    feature = "client",
    doc = "
```rust,no_run
fn main() {
    let string = \"Hello this GrammarBot world!\";
    let mut r = grammarbot_io::Request::from(string);
    // With an API key:
    println!(\"Response: {:#?}\", r.api_key(\"99999999\").send());
    // Without an API key:
    println!(\"Response: {:#?}\", r.send());
}
```"
)]
#![deny(missing_docs)]
#![deny(warnings)]

#[cfg(feature = "client")]
/// The client feature implementation.
pub mod client;
/// The request structures.
pub mod request;
/// The response structures.
pub mod response;

pub use request::{HttpRequest, Request};
pub use response::Response;

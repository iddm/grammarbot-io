//! GrammarBot crate - check your sentence for grammar.
//!
//! This crate uses [official API](https://www.grammarbot.io/) of
//! [the GrammarBot service](http://grammarbot.io/).
//!
//! The service has rate limits:
//! With an API key, you can perform 100 requests a day or around 3000 a month for free.
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
    println!(\"Response: {:#?}\", r.api_key(\"99999999\").send());
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

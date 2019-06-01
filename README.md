[![Build status](https://travis-ci.org/vityafx/grammarly-rs.svg?branch=master)](https://travis-ci.org/vityafx/grammarly-rs)
[![Crates](https://img.shields.io/crates/v/grammarly.svg)](https://crates.io/crates/grammarly)
[![Docs](https://docs.rs/grammarly/badge.svg)](https://docs.rs/grammarly)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# Grammarly
Easy to use API for performing requests to the Grammarly API for checking your grammar in your sentences.

# Usage

1. [Obtain the API key](https://www.grammarbot.io/signup)
2. Use the library:

```rust,no_run
fn main() {
    let string = "Hello this grammarly world!";
    let mut r = grammarly::Request::from(string);
    // With an API key:
    println!("Response: {:#?}", r.api_key("99999999").send());
    // Without an API key:
    println!("Response: {:#?}", r.send());
}
```

# Features
The crate contains the only one feature: **client** which currently uses
`reqwest` crate to perform requests. If you don't want to use this feature,
or you don't want the crate to fetch the network-related dependencies and link
against them, you may perform the requests yourself, using the `HttpRequest`
structure, which contains all necessary information to perform the request, and
can be easily created from the `grammarly::Request` object.

# License
[This project is licensed under the MIT license.](LICENSE)

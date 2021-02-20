[![Build status](https://api.travis-ci.com/vityafx/grammarbot-io.svg?branch=master)](https://travis-ci.com/vityafx/grammarbot-io)
[![Crates](https://img.shields.io/crates/v/grammarbot-io.svg)](https://crates.io/crates/grammarbot-io)
[![Docs](https://docs.rs/grammarbot-io/badge.svg)](https://docs.rs/grammarbot-io)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

# Grammarbot.io
Easy to use API for performing requests to the [GrammarBot.io](https://grammarbot.io) API for checking your grammar in your sentences.

# Usage

1. [Obtain the API key](https://www.grammarbot.io/signup)
2. Use the library:

```rust,no_run
fn main() {
    let string = "Hello this grammarbot-io world!";
    let mut r = grammarbot-io::Request::from(string);
    // With an API key:
    println!("Response: {:#?}", r.api_key("99999999").send());
}
```

# Examples
You may run an example which uses an environment variable `API_KEY`:

```rust,no_run
fn main() {
    use std::env;

    let string = "Hello this grammarbot-io world!";
    let mut r = grammarbot-io::Request::from(string);
    // With an API key:
    println!(
        "Response: {:#?}",
        r.api_key(env::var("API_KEY").expect("The API_KEY variable should be set."))
            .send()
    );
}
```

by running

```
API_KEY=<YOUR API KEY> cargo run --example run_with_env
```

# Features
The crate contains the only one feature: **client** which currently uses
`reqwest` crate to perform requests. If you don't want to use this feature,
or you don't want the crate to fetch the network-related dependencies and link
against them, you may perform the requests yourself, using the `HttpRequest`
structure, which contains all necessary information to perform the request, and
can be easily created from the `grammarbot-io::Request` object.

# License
[This project is licensed under the MIT license.](LICENSE)

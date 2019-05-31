[![Build Status](https://drone.thefx.co/api/badges/fx/grammarly-rs/status.svg)](https://drone.thefx.co/fx/grammarly-rs)

# Grammarly
Easy to use API for performing requests to the Grammarly API for checking your grammar in your sentences.

# Usage

1. [Obtain the API key](https://www.grammarbot.io/signup)
2. Use the library (TBA).

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

# License
[This project is licensed under the MIT license.](LICENSE)

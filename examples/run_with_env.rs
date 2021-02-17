fn main() {
    use std::env;

    let string = "Hello this grammarly world!";
    let mut r = grammarly::Request::from(string);
    // With an API key:
    println!(
        "Response: {:#?}",
        r.api_key(env::var("API_KEY").expect("The API_KEY variable should be set."))
            .send()
    );
}

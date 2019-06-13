//! The grammarly request.

/// The URL to send the requests to.
pub const GRAMMARLY_CHECK_URL: &str = "http://api.grammarbot.io/v2/check";

/// Grammarly's api key strong type.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ApiKey(pub String);

impl<T> From<T> for ApiKey
where
    T: AsRef<str>,
{
    fn from(s: T) -> ApiKey {
        ApiKey(s.as_ref().to_owned())
    }
}

/// Represents an English language variation.
#[derive(Debug, Clone, serde::Serialize)]
pub enum EnglishLanguageVariation {
    /// GB
    British,
    /// US
    American,
}

/// Represents the language to be passed to the service.
#[derive(Debug, Clone, serde::Serialize)]
pub enum Language {
    /// The English language with variations.
    English(EnglishLanguageVariation),
}

impl Language {
    /// Serializes the object for passing inside a request.
    pub fn to_short_string(&self) -> &'static str {
        match self {
            Language::English(EnglishLanguageVariation::British) => "en-GB",
            Language::English(EnglishLanguageVariation::American) => "en-US",
        }
    }
}

impl Default for Language {
    fn default() -> Language {
        Language::English(EnglishLanguageVariation::British)
    }
}

/// The request object. Used to send data to the service.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Request {
    api_key: ApiKey,
    text: String,
    language: Language,
}

impl<T> From<T> for Request
where
    T: AsRef<str>,
{
    fn from(s: T) -> Request {
        Request {
            text: s.as_ref().to_owned(),
            ..Default::default()
        }
    }
}

impl Request {
    /// Mutates the object setting the api key.
    /// Not necessary to use as service allows performing requests without one.
    pub fn api_key<T: Into<ApiKey>>(&mut self, key: T) -> &mut Request {
        self.api_key = key.into();
        self
    }

    /// Mutates the object setting the language to check grammar for.
    pub fn language<T: Into<Language>>(&mut self, language: T) -> &mut Request {
        self.language = language.into();
        self
    }
}

/// An HTTP method.
#[derive(Debug, Clone)]
pub enum HttpRequestMethod {
    /// GET HTTP method.
    Get,
    /// POST HTTP method.
    Post,
}

/// The http request object. Used to pack grammarly request so that
/// it can be used to send requests from other request crates (`reqwest`/`hyper`/etc).
/// The main use case is when the library is built without the `client` feature, so
/// you still can perform requests using your own implementation.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// Request URL.
    pub url: String,
    /// HTTP request method type.
    pub method: HttpRequestMethod,
    /// Key-value pairs of the request data.
    pub values: Vec<(String, String)>,
}

impl From<&Request> for HttpRequest {
    fn from(r: &Request) -> HttpRequest {
        HttpRequest {
            url: GRAMMARLY_CHECK_URL.to_owned(),
            method: HttpRequestMethod::Get,
            values: vec![
                ("api_key".to_owned(), r.api_key.0.clone()),
                (
                    "language".to_owned(),
                    r.language.to_short_string().to_owned(),
                ),
                ("text".to_owned(), r.text.clone()),
            ],
        }
    }
}

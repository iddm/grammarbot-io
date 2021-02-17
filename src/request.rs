//! The grammarly request.

/// The URL to send the requests to.
pub const GRAMMARLY_CHECK_URL: &str = "https://grammarbot.p.rapidapi.com/check";

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

/// The request data. Used to send data to the service.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct RequestData {
    /// The language we need to check for.
    pub language: String,
    /// The text we are sending to the service for checking.
    pub text: String,
}

/// The request object. Used to send data to the service.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct Request {
    api_key: ApiKey,
    data: RequestData,
}

impl<T> From<T> for Request
where
    T: AsRef<str>,
{
    fn from(s: T) -> Request {
        Request {
            data: RequestData {
                text: s.as_ref().to_owned(),
                ..Default::default()
            },
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
        self.data.language = language.into().to_short_string().to_owned();
        self
    }
}

/// The http request object. Used to pack grammarly request so that
/// it can be used to send requests from other request crates (`reqwest`/`hyper`/etc).
/// The main use case is when the library is built without the `client` feature, so
/// you still can perform requests using your own implementation.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// Request URL.
    pub url: String,
    /// Key-value pairs of the request headers.
    pub headers: Vec<(String, String)>,
    /// Key-value pairs of the request data.
    pub data: RequestData,
}

impl From<&Request> for HttpRequest {
    fn from(r: &Request) -> HttpRequest {
        HttpRequest {
            url: GRAMMARLY_CHECK_URL.to_owned(),
            headers: vec![
                (
                    "x-rapidapi-host".to_owned(),
                    "grammarbot.p.rapidapi.com".to_owned(),
                ),
                ("x-rapidapi-key".to_owned(), r.api_key.0.clone()),
                ("useQueryString".to_owned(), "true".to_owned()),
            ],
            data: r.data.clone(),
        }
    }
}

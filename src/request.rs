//! The grammarly request.

/// The URL to send the requests to.
pub static GRAMMARLY_CHECK_URL: &'static str = "http://api.grammarbot.io/v2/check";

/// Grammarly's api key strong type.
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ApiKey(pub String);

impl From<&str> for ApiKey {
    fn from(s: &str) -> ApiKey {
        ApiKey(s.to_owned())
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

impl From<&str> for Request {
    fn from(s: &str) -> Request {
        Request {
            text: s.to_owned(),
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

    /// Sets the language to check grammar for.
    pub fn language<T: Into<Language>>(&mut self, language: T) -> &mut Request {
        self.language = language.into();
        self
    }
}

#[cfg(feature = "client")]
impl Request {
    /// Sends the request and returns the response using default client.
    pub fn send(&self) -> Result<crate::response::Response, reqwest::Error> {
        self.send_with_client(&reqwest::Client::new())
    }

    /// Sends the request using the provided client.
    pub fn send_with_client(
        &self,
        client: &reqwest::Client,
    ) -> Result<crate::response::Response, reqwest::Error> {
        client
            .get(GRAMMARLY_CHECK_URL)
            .query(&[
                ("api_key", self.api_key.0.clone()),
                ("language", self.language.to_short_string().to_owned()),
                ("text", self.text.clone()),
            ])
            .send()?
            .json()
    }
}

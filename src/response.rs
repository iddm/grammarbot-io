//! Response module contains all the structures necessary
//! for doing all the interesting stuff.

/// Represents a strong type on the grammarly bot version.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct GrammarlyVersion(pub String);

/// Represents a strong type on the grammarly api version.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct ApiVersion(pub u64);

/// Represents a bot info.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Software {
    /// The name of the bot.
    pub name: String,
    /// The grammarly's bot version
    pub version: GrammarlyVersion,
    /// The api version used.
    #[serde(rename = "apiVersion")]
    pub api_version: ApiVersion,
    /// Is true if the API key used is premium one.
    pub premium: bool,
    /// Contains premium hint. If the API key is not a premium one,
    /// suggests to get one.
    #[serde(rename = "premiumHint")]
    pub premium_hint: String,
    /// Grammarly bot check status.
    pub status: String,
}

/// Represents warnings in the response. They are useful
/// but could be ignored (not necessarily fixed) and they
/// should be treated as an advice.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Warnings {
    /// True if results are incompleted.
    #[serde(rename = "incompleteResults")]
    pub incomplete_results: bool,
    // TODO Check for other values.
}

/// Detected language code and name, determined by the
/// grammarly bot.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct DetectedLanguage {
    /// The language name.
    pub name: String,
    /// The language code.
    pub code: String,
}

/// Passed language and detected language information.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct LanguageDetails {
    /// The language name.
    pub name: String,
    /// The language code.
    pub code: String,
    /// The language detected by the grammarly bot.
    #[serde(rename = "detectedLanguage")]
    pub detected_language: DetectedLanguage,
}

/// Represents a single possible replacement of the word.
/// If you have mispelled a word, the service advises you
/// to correct the word and all the corrections are suggested
/// using these `Replacement` objects.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Replacement {
    /// The suggested word to change to.
    pub value: String,
}

/// Represents the problem context, a sentence, or a subsentence.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Context {
    /// The context itself.
    pub text: String,
    /// The word's offset causing the problem inside the context.
    pub offset: u64,
    /// The length of the word in the context.
    pub length: u64,
}

/// Represents a single problem found.
#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct Match {
    /// Problem detailed explanation.
    pub message: String,
    /// Short explanation, the type of a problem.
    #[serde(rename = "shortMessage")]
    pub short_message: String,
    /// All the replacement suggestions.
    pub replacements: Vec<Replacement>,
    /// Offset from the beginning of the passed text.
    pub offset: u64,
    /// The length of the word causing the problem.
    pub length: u64,
    /// The context in which the problem was found.
    pub context: Context,
    /// The whole sentence where the problem was found.
    pub sentence: String,
    // TODO specify other fields...
}

/// Grammarly's response structure.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum Response {
    /// A successful response.
    Success {
        /// Grammarly bot information.
        software: Software,
        /// Grammarly bot warnings.
        warnings: Warnings,
        /// Language information.
        language: LanguageDetails,
        /// Problems found.
        matches: Vec<Match>,
    },
    /// Not a successful response.
    Failure {
        /// Contains text explaining the problem.
        message: String,
    },
}

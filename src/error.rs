use std::str::Utf8Error;

// ANOTHER todo: more params in the errors


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no cookie found")]
    NoCookieFound,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("Could not decode the response bytes: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Invalid response from Cleverbot API")]
    InvalidResponseFromCleverbotApi,
    #[error("Bad response from Cleverbot API: {0}")]
    BadResponse(String),
    #[error("Bad response from Cleverbot API after retrying: {0}")]
    BadResponseAfterRetrying(String),
}

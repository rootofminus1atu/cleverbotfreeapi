use std::str::Utf8Error;


#[derive(Debug, thiserror::Error)]
/// An enum representing the different types of errors that can occur in the Cleverbot client.
pub enum Error {
    /// Indicates that no cookie was found. Could occur during the initial Cleverbot struct creation, or when retrying.
    #[error("no cookie found")]
    NoCookieFound,
    
    /// Indicates an error with reqwest. Should not occur.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    
    /// Indicates that a cleverbot response could not be decode. Should not occur.
    #[error("Could not decode the response bytes: {0}")]
    Utf8Error(#[from] Utf8Error),
    
    /// Indicates that the response from a cleverbot response was invalid. Should not occur.
    #[error("Invalid response from Cleverbot API")]
    InvalidResponseFromCleverbotApi,
    
    /// Indicates that a bad response was received from cleverbot. A bad response usually means that the cookie expired and so a retry with a new cookie would then be attempted.
    #[error("Bad response from Cleverbot API: {0}")]
    BadResponse(String),
    
    /// Indicates that a bad response was received from cleverbot after retrying. Same as above, but a new cookie didn't work. Oh well, too bad!
    #[error("Bad response from Cleverbot API after retrying: {0}")]
    BadResponseAfterRetrying(String),
}

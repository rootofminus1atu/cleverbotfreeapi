//! the main doc thing
//! 
//! Cleverbot 


mod cleverbot;
mod error;
mod helpers;
mod cookie_generation;

pub use error::Error;
pub use cleverbot::Cleverbot;
pub use cleverbot::CleverbotBuilder;
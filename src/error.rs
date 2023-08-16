//! The main error enum for the project lives here, and documents the various
//! conditions that can arise while interacting with the system.

use thiserror::Error;

/// This is our error enum. It contains an entry for any part of the system in
/// which an expectation is not met or a problem occurs.
#[derive(Error, Debug)]
pub enum Error {
    /// An error while dealing with local storage.
    #[error("Local storage error: {0}")]
    LocalStorage(String),
}

/// Wraps `std::result::Result` around our `Error` enum
pub type Result<T> = std::result::Result<T, Error>;


//! Chain client errors
use std::fmt;

use failure::{Backtrace, Context, Fail};

/// Alias of `Result` objects that return [`Error`]
/// 
/// [`Error`]: self::Error
pub type Result<T> = std::result::Result<T, Error>;

/// An opaque error type, used for all errors in this crate
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

/// Different variants of possible errors
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    /// Storage initialization error
    #[fail(display = "Storage initialization error")]
    StorageInitializationError,
    /// Other storage error
    #[fail(display = "Storage error")]
    StorageError,
    /// Key generation error
    #[fail(display = "Key generation error")]
    KeyGenerationError,
    /// Random number generator error
    #[fail(display = "Random number generator error")]
    RngError,
    /// Decryption error
    #[fail(display = "Decryption error")]
    DecryptionError,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error {
    /// Returns [`ErrorKind`] of current error
    ///
    /// [`ErrorKind`]: self::ErrorKind
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}

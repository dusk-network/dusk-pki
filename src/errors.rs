use std::io;
use thiserror::Error;

/// Errors for Dusk PKI
#[derive(Error, Debug)]
pub enum Error {
    /// Invalid Compressed Point
    #[error("Invalid Compressed Point Provided")]
    InvalidPoint,
    #[error("Invalid Parameters provided to the function")]
    /// Invalid Parameters
    InvalidParameters,
    /// Bad Length
    #[error("Bad Length (expected {expected:?}, got {found:?})")]
    BadLength {
        /// The found length
        found: usize,
        /// The expected length
        expected: usize,
    },
    // TODO: this should be handled better, it's too generic
    #[doc(hidden)]
    #[error("Invalid I/O operation")]
    Io(#[from] io::Error),
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        match err {
            _ => io::Error::new(io::ErrorKind::Other, format!("{}", err)),
        }
    }
}

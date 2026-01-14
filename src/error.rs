use std::fmt;

/// The error type for the Lemonaid library.
#[derive(Debug)]
pub enum LemonaidError {
    /// An HTTP/network error from the underlying reqwest client.
    Http(reqwest::Error),
    /// An API error response with status code and optional message body.
    Api {
        status: reqwest::StatusCode,
        message: String,
    },
}

impl fmt::Display for LemonaidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LemonaidError::Http(err) => write!(f, "HTTP error: {}", err),
            LemonaidError::Api { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
        }
    }
}

impl std::error::Error for LemonaidError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LemonaidError::Http(err) => Some(err),
            LemonaidError::Api { .. } => None,
        }
    }
}

impl From<reqwest::Error> for LemonaidError {
    fn from(err: reqwest::Error) -> Self {
        LemonaidError::Http(err)
    }
}

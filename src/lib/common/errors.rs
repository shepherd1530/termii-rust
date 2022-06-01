use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum HttpError {
    #[error("Invalid timeout value {0}")]
    InvalidTimeout(u64),

    #[error("Missing query params. API keys will be passed with query params on get requests.")]
    MissingQueryParams,

    #[error("Unable to parse the given url. {0}")]
    UrlParseError(String),

    #[error("Termii search item response error.")]
    SearchItemError {
        number: String,
        message: String,
        status: String,
        dnd_active: bool,
        network: String,
        network_code: String,
    },

    #[error("Unexpected response status code {status:?} {message:?}.")]
    JsonError { status: usize, message: String },

    #[error("Termii status item response error. {0}")]
    NetworkError(String),

    #[error(transparent)]
    Io(#[from] ReqwestError),
}

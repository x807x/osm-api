use std::fmt::Display;

use quick_xml::DeError;

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Reqwest(reqwest::Error),
    Http(reqwest::StatusCode),
    XML(DeError),
    #[cfg(feature = "json-api")]
    JSON(serde_json::Error),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO error: {}", e),
            Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Error::Http(status) => write!(f, "HTTP status: {}", status),
            Error::XML(e) => write!(f, "Deserialize error: {}", e),
            #[cfg(feature = "json-api")]
            Error::JSON(e) => write!(f, "JSON error: {}", e),
            Error::Other(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<DeError> for Error {
    fn from(e: DeError) -> Self {
        Error::XML(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JSON(e)
    }
}

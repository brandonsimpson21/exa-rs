use reqwest::Error as ReqwestError;
use thiserror::Error;

//implement keyword search
#[derive(Debug, Error)]
pub enum ExaApiError {
    #[error("network error: {0}")]
    NetworkError(#[from] ReqwestError),
    #[error("API error: {0}")]
    ApiError(String),
}

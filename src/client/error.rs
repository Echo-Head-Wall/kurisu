use jira_openapi::apis::Error as OpenApiError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing base URL for Jira instance")]
    MissingBaseUrl,

    #[error("HTTP client error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Middleware client error: {0}")]
    ReqwestMiddlewareError(#[from] reqwest_middleware::Error),

    #[error("API request error: {0}")]
    ApiError(String),

    #[error("JSON parsing error: {0}")]
    SerdeError(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("{0}")]
    Other(String),
}

impl<T> From<OpenApiError<T>> for Error 
where
    T: std::fmt::Debug,
{
    fn from(error: OpenApiError<T>) -> Self {
        match error {
            OpenApiError::Reqwest(e) => Self::ReqwestError(e),
            OpenApiError::ReqwestMiddleware(e) => Self::ReqwestMiddlewareError(e),
            OpenApiError::Serde(e) => Self::SerdeError(e),
            OpenApiError::Io(e) => Self::IoError(e),
            OpenApiError::ResponseError(resp) => {
                Self::ApiError(format!(
                    "Response error: status={}, content={}", 
                    resp.status, 
                    resp.content
                ))
            }
        }
    }
}
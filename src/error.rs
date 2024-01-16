#[derive(Debug, Clone)]
pub enum AppError {
    ReqwestError(String),
    SerdeError(String)
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value.to_string())
    }
}
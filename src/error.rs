#[derive(Debug, Clone)]
pub enum AppError {
    ReqwestError(String),
    NonInitiated
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::ReqwestError(value)
    }
}

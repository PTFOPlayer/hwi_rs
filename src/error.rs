#[derive(Debug)]
pub enum AppError {
    ReqwestError(String),
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self::ReqwestError(value)
    }
}

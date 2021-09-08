#[derive(Debug)]
pub enum FastError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    ParseIntError(std::num::ParseIntError),
}
impl From<reqwest::Error> for FastError {
    fn from(e: reqwest::Error) -> Self {
        FastError::ReqwestError(e)
    }
}
impl From<serde_json::Error> for FastError {
    fn from(e: serde_json::Error) -> Self {
        FastError::SerdeError(e)
    }
}
impl From<std::io::Error> for FastError {
    fn from(e: std::io::Error) -> Self {
        FastError::IoError(e)
    }
}
impl From<std::num::ParseIntError> for FastError {
    fn from(e: std::num::ParseIntError) -> Self {
        FastError::ParseIntError(e)
    }
}
impl std::fmt::Display for FastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            FastError::ReqwestError(e) => e.fmt(f),
            FastError::SerdeError(e) => e.fmt(f),
            FastError::IoError(e) => e.fmt(f),
            FastError::ParseIntError(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for FastError {}

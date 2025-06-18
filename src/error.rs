use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpdbError {
    #[error("Null pointer encountered")]
    NullPointer,
    #[error("Invalid printer object")]
    InvalidPrinter,
    #[error("Print job failed: {0}")]
    JobFailed(String),
    #[error("Backend error: {0}")]
    BackendError(String),
    // Add more error variants as needed
}

pub type Result<T> = std::result::Result<T, CpdbError>;
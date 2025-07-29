use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Git(String),
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {e}"),
            AppError::Git(msg) => write!(f, "Git error: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::Io(error)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

pub mod commit_types;
pub mod git;
pub mod prompts;
pub mod semver;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_app_error_display() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_error = AppError::Io(io_error);
        assert!(format!("{}", app_error).contains("file not found"));

        let git_error = AppError::Git("not a git repository".to_string());
        assert_eq!(format!("{}", git_error), "Git error: not a git repository");

        let invalid_input = AppError::InvalidInput("empty message".to_string());
        assert_eq!(format!("{}", invalid_input), "Invalid input: empty message");
    }

    #[test]
    fn test_app_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let app_error: AppError = io_error.into();
        match app_error {
            AppError::Io(_) => {}
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_error_chain() {
        let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_error = AppError::Io(io_error);
        match app_error {
            AppError::Io(ref inner) => assert!(inner.source().is_none()),
            _ => panic!("Expected Io variant"),
        }
    }

    #[test]
    fn test_result_type() {
        let success: Result<i32> = Ok(42);
        assert_eq!(success.unwrap(), 42);

        let failure: Result<i32> = Err(AppError::InvalidInput("test error".to_string()));
        assert!(failure.is_err());
    }
}

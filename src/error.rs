#[derive(thiserror::Error, Debug)]
pub enum DdError {
    #[error("Absolute paths are not supported: {path:?}")]
    AbsolutePath { path: std::path::PathBuf },
    #[error("Config file not found")]
    ConfigNotFound,
    #[error("Invalid config")]
    InvalidConfig,
    #[error("Fmt Error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("Hjson Error: {0}")]
    Hjson(#[from] deser_hjson::Error),
    #[error("Init not possible: {0}")]
    InitNotPossible(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Invalid page path: {path}")]
    InvalidPagePath { path: String },
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Failed to read {path:?}: {error}")]
    Read {
        path: std::path::PathBuf,
        error: std::io::Error,
    },
    #[error("Server error: {0}")]
    Server(String),
    #[error("TOML Error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Unsupported file format: {0}")]
    UnsupportedFileFormat(std::path::PathBuf),
    #[error("Invalid UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

pub type DdResult<T> = Result<T, DdError>;

impl DdError {
    pub fn internal<S: Into<String>>(msg: S) -> Self {
        DdError::Internal(msg.into())
    }
}

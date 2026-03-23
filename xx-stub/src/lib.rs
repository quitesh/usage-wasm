use miette::Diagnostic;
use std::path::{Path, PathBuf};
use thiserror::Error;

// ── Error types ─────────────────────────────────────────────────────────────

#[derive(Error, Diagnostic, Debug)]
pub enum XXError {
    #[error("{0}")]
    Error(String),

    #[error("{0}\nFile: {1}")]
    #[diagnostic(code(xx::file), url(docsrs))]
    FileError(std::io::Error, PathBuf),

    #[error("{0}\n{1}")]
    #[diagnostic(code(xx::process), url(docsrs))]
    ProcessError(std::io::Error, String),
}

pub type XXResult<T> = Result<T, XXError>;

// ── Error module (re-export for xx::error::XXError path) ────────────────────

pub mod error {
    pub use super::{XXError, XXResult};
}

// ── File module ─────────────────────────────────────────────────────────────

pub mod file {
    use super::*;

    pub fn read_to_string<P: AsRef<Path>>(path: P) -> XXResult<String> {
        Err(XXError::FileError(
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "filesystem access not available in wasm",
            ),
            path.as_ref().to_path_buf(),
        ))
    }
}

// ── Process module ──────────────────────────────────────────────────────────

pub mod process {
    pub fn check_status(_status: std::process::ExitStatus) -> std::io::Result<()> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "process operations not available in wasm",
        ))
    }
}

// ── Regex macro ─────────────────────────────────────────────────────────────

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

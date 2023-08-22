use indicatif::style::TemplateError;
use reqwest::Error as HtmlError;
use rusqlite::Error as SqlError;
use serde_json::Error as JsonError;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Error as FmtError;
use std::io::Error as IoError;

#[derive(Debug)]
pub struct NoResults;
impl std::fmt::Display for NoResults {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No results found")
    }
}
#[derive(Debug)]
pub enum AppError {
    Sqlite(SqlError),
    Json(JsonError),
    Html(HtmlError),
    Io(IoError),
    Fmt(FmtError),
    Box(Box<dyn StdError>),
    Template(TemplateError),
    NoResults,
}
impl From<SqlError> for AppError {
    fn from(err: SqlError) -> Self {
        AppError::Sqlite(err)
    }
}
impl From<JsonError> for AppError {
    fn from(err: JsonError) -> Self {
        AppError::Json(err)
    }
}
impl From<HtmlError> for AppError {
    fn from(err: HtmlError) -> Self {
        AppError::Html(err)
    }
}
impl From<IoError> for AppError {
    fn from(err: IoError) -> Self {
        AppError::Io(err)
    }
}
impl From<FmtError> for AppError {
    fn from(err: FmtError) -> Self {
        AppError::Fmt(err)
    }
}
impl From<TemplateError> for AppError {
    fn from(err: TemplateError) -> Self {
        AppError::Template(err)
    }
}
impl From<Box<dyn StdError>> for AppError {
    fn from(err: Box<dyn StdError>) -> Self {
        AppError::Box(err)
    }
}
impl StdError for NoResults {
    fn description(&self) -> &str {
        "No results found"
    }
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Sqlite(error) => write!(f, "Rusqlite error: {}", error),
            AppError::Json(error) => write!(f, "serde_json error: {}", error),
            AppError::Html(error) => write!(f, "Reqwest error: {}", error),
            AppError::Io(error) => write!(f, "IO error: {}", error),
            AppError::Fmt(error) => write!(f, "Fmt error: {}", error),
            AppError::Template(error) => write!(f, "Progress bar template error: {}", error),
            AppError::Box(error) => write!(f, "Std Error: {}", error),
            AppError::NoResults => write!(f, "No results found"),
        }
    }
}

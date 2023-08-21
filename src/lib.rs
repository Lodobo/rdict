pub mod utils {
    use std::{error::Error, path::PathBuf};
    pub fn get_home_directory() -> Result<PathBuf, Box<dyn Error>> {
        let home_dir = match dirs::home_dir() {
            Some(path) => path,
            None => {
                return Err("Failed to retrieve home directory".into());
            }
        };
        Ok(home_dir)
    }
}

pub mod format {
    pub fn panel(word_1: &str, word_2: &str) -> String {
        format!(
            "╭─{}─╮   ╭─{}─╮\n\
             │ {} │   │ {} │\n\
             ╰─{}─╯   ╰─{}─╯",
            "─".repeat(word_1.len()),
            "─".repeat(word_2.len()),
            word_1,
            word_2,
            "─".repeat(word_1.len()),
            "─".repeat(word_2.len())
        )
    }
    pub fn wrap_text(paragraph: &str, max_width: usize, indent_length: usize) -> String {
        let words = paragraph.split_whitespace().collect::<Vec<&str>>();
        let indent = " ".repeat(indent_length);
        let mut space_left = max_width;
        words
            .iter()
            .enumerate()
            .fold(String::new(), |mut wrapped, (i, word)| {
                if i == 0 {
                    wrapped.push_str(&format!("{}{}", indent, word));
                    space_left -= word.len();
                } else if word.len() + 1 > space_left {
                    wrapped.push('\n');
                    wrapped.push_str(&format!("{}{}", indent, word));
                    space_left = max_width.saturating_sub(word.len());
                } else {
                    if !wrapped.ends_with('\n') {
                        wrapped.push(' ');
                        space_left -= 1;
                    }
                    wrapped.push_str(word);
                    space_left -= word.len();
                }
                wrapped
            })
    }
}

pub mod structs {
    use serde::{Deserialize, Serialize};
    use std::borrow::Cow;

    #[derive(Serialize, Deserialize)]
    pub struct Word<'a> {
        #[serde(borrow)]
        pub word: Cow<'a, str>,
        #[serde(borrow)]
        pub pos: Cow<'a, str>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct WordInfo<'a> {
        pub senses: Vec<Sense<'a>>,
        #[serde(borrow)]
        pub etymology_text: Option<Cow<'a, str>>,
        pub sounds: Option<Vec<Sound<'a>>>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Sound<'a> {
        #[serde(borrow)]
        pub ipa: Option<Cow<'a, str>>,
        #[serde(borrow)]
        pub tags: Option<Vec<Cow<'a, str>>>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Sense<'a> {
        #[serde(borrow)]
        pub glosses: Option<Vec<Cow<'a, str>>>,
        pub examples: Option<Vec<Example<'a>>>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Example<'a> {
        #[serde(borrow)]
        pub text: Option<Cow<'a, str>>,
    }

    pub struct Row {
        pub word: String,
        pub pos: String,
        pub information: Option<String>,
    }
}

pub mod error {
    use rusqlite::Error as SqlError;
    use serde_json::Error as JsonError;
    use std::error::Error as StdError;
    use std::fmt;
    use std::fmt::Error as FmtError;
    use std::io::Error as IoError;
    use reqwest::Error as HtmlError;

    #[derive(Debug)]
    pub enum AppError {
        Sqlite(SqlError),
        Json(JsonError),
        Html(HtmlError),
        Io(IoError),
        Fmt(FmtError),
        Box(Box<dyn StdError>),
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
    impl From<Box<dyn StdError>> for AppError {
        fn from(err: Box<dyn StdError>) -> Self {
            AppError::Box(err)
        }
    }

    #[derive(Debug)]
    pub struct NoResults;
    impl std::fmt::Display for NoResults {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "No results found")
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
                AppError::Box(error) => write!(f, "Std Error: {}", error),
                AppError::NoResults => write!(f, "No results found"),

            }
        }
    }
}

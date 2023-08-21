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

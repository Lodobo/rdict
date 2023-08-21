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

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Word {
        pub word: String,
        pub pos: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Information {
        pub senses: Vec<Sense>,
        pub etymology_text: Option<String>,
        pub sounds: Option<Vec<Sound>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Sound {
        pub ipa: Option<String>,
        pub tags: Option<Vec<String>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Sense {
        pub glosses: Option<Vec<String>>,
        pub examples: Option<Vec<Example>>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Example {
        pub text: Option<String>,
        pub reference: Option<String>,
    }

    pub struct Row {
        pub word: String,
        pub pos: String,
        pub information: Option<String>,
    }
}

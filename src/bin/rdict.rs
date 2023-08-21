use ansi_term::Style;
use clap::Parser;
use pager_rs::{CommandList, State, StatusBar};
use rdict::{
    error::AppError,
    format::{panel, wrap_text},
    structs::{Row, WordInfo},
};
use rusqlite::Connection;
use std::fmt::Write;

// Define the command-line arguments
#[derive(Parser)]
#[command(name = "rdict")]
#[command(author = "Lodobo. <lodobo.n8qbt@simplelogin.com>")]
#[command(version = "1.0")]
#[command(about = "Offline CLI dictionary")]
struct Cli {
    /// Search word
    #[arg(short, long)]
    word: String,
}

fn main() -> Result<(), AppError> {
    let mut output = String::new();
    for row in sql_query()? {
        print_word_information(&mut output, &row)?;
    }
    let mut state = State::new(
        output,
        StatusBar::new("rdict".to_string()),
        CommandList::default(),
    )?;
    state.show_line_numbers = false;
    pager_rs::init()?;
    pager_rs::run(&mut state)?;
    pager_rs::finish()?;
    Ok(())
}

// Function to format and print word information
fn print_word_information(output: &mut String, row: &Row) -> Result<(), AppError> {
    // Print Panel (Part of speech + Word)
    write!(
        output,
        "\n{}\n",
        Style::new()
            .bold()
            .paint(panel(&row.pos.to_uppercase(), &row.word))
    )?;
    if let Some(information_json) = &row.information {
        let information: WordInfo = serde_json::from_str(information_json)?;
        // Print Pronunciation
        if let Some(sounds) = &information.sounds {
            write!(
                output,
                "\n{}\n\n",
                Style::new().bold().paint("# Pronunciation")
            )?;
            for pronunciation in sounds {
                if let Some(ipa) = &pronunciation.ipa {
                    write!(output, "  {}", ipa)?;
                    if let Some(tags) = &pronunciation.tags {
                        write!(output, " ({})", tags[0])?;
                    }
                    writeln!(output)?;
                }
            }
        }
        // Print Etymology
        if let Some(etymology) = &information.etymology_text {
            if etymology.len() > 0 {
                write!(output, "\n{}\n\n", Style::new().bold().paint("# Etymology"))?;
                writeln!(output, "{}", wrap_text(etymology, 90, 2))?;
            }
        }
        // Print Definitions
        write!(output, "\n{}\n", Style::new().bold().paint("# Definitions"))?;
        for definition in &information.senses {
            for def in definition.glosses.as_ref().unwrap() {
                write!(
                    output,
                    "\n{}\n",
                    Style::new().bold().paint(wrap_text(def, 90, 2))
                )?;
            }
            if let Some(examples) = &definition.examples {
                for example in examples {
                    if let Some(text) = &example.text {
                        write!(output, "\n{}\n", wrap_text(&format!("\"{}\"", text), 84, 6))?;
                    }
                }
            }
        }
    }
    Ok(())
}

// Function to execute the SQL query and retrieve word information
fn sql_query() -> Result<Vec<Row>, AppError> {
    let path_to_db = rdict::utils::get_home_directory()?.join(".local/share/rdict/en.db");
    let conn = Connection::open(path_to_db)?;
    let query_word = Cli::parse().word;
    let mut stmt = conn.prepare("SELECT word, pos, information FROM en WHERE word = ?1;")?;
    let row_iter = stmt.query_map([&query_word], |row| {
        Ok(Row {
            word: row.get(0)?,
            pos: row.get(1)?,
            information: row.get(2)?,
        })
    })?;
    let results: Vec<Row> = row_iter.filter_map(Result::ok).collect();

    if results.is_empty() {
        return Err(AppError::NoResults);
    }
    Ok(results)
}

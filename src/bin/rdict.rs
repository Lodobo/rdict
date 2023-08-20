use ansi_term::Style;
use clap::Parser;
use pager_rs::{CommandList, State, StatusBar};
use rdict::{
    format::{panel, wrap_text},
    structs,
};
use rusqlite::{Connection, Result};
use std::{error::Error, fmt::Write};

#[derive(Parser)]
#[command(name = "rdict")]
#[command(author = "Lodobo. <lodobo.n8qbt@simplelogin.com>")]
#[command(version = "1.0")]
#[command(about = "Offline CLI dictionary", long_about = None)]
struct Cli {
    /// Search word
    #[arg(short, long)]
    word: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    let cli = Cli::parse();
    let home_dir = rdict::utils::get_home_directory()?;
    let rdict_dir = home_dir.join(".local/share/rdict");
    let path_to_db = &rdict_dir.join("en.db");
    let conn = Connection::open(path_to_db)?;
    let rows = sql_query(&conn, &cli.word)?;
    for row in rows {
        print_word_information(&mut output, &row)?;
    }
    let status_bar = StatusBar::new(format!("rdict -w {}", &cli.word));
    let mut state = State::new(output, status_bar, CommandList::default())?;
    state.show_line_numbers = false;
    pager_rs::init()?;
    pager_rs::run(&mut state)?;
    pager_rs::finish()?;
    Ok(())
}

fn print_word_information(output: &mut String, row: &structs::Row) -> Result<(), Box<dyn Error>> {
    // Print Panel
    let panel = panel(&row.pos.to_uppercase(), &row.word);
    write!(output, "\n{}\n", Style::new().bold().paint(panel))?;
    if let Some(information_json) = &row.information {
        let information: structs::Information = serde_json::from_str(information_json)?;

        // Print Pronunciation
        if let Some(sounds) = &information.sounds {
            let title = "# Pronunciation";
            write!(output, "\n{}\n\n", Style::new().bold().paint(title))?;
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
            let title = "# Etymology";
            write!(output, "\n{}\n\n", Style::new().bold().paint(title))?;
            writeln!(output, "{}", wrap_text(etymology, 90, 2))?;
        }
        // Print Definitions
        let title = "# Definitions";
        write!(output, "\n{}\n", Style::new().bold().paint(title))?;
        for definition in &information.senses {
            for def in definition.glosses.as_ref().unwrap() {
                write!(
                    output,
                    "\n-{}\n",
                    Style::new().bold().paint(wrap_text(def, 90, 1))
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

fn sql_query(conn: &Connection, query_word: &str) -> Result<Vec<structs::Row>> {
    let mut stmt = conn.prepare("SELECT word, pos, information FROM en where word=?1;")?;
    let row_iter = stmt.query_map([&query_word], |row| {
        Ok(structs::Row {
            word: row.get(0)?,
            pos: row.get(1)?,
            information: row.get(2)?,
        })
    })?;
    Ok(row_iter.filter_map(Result::ok).collect())
}

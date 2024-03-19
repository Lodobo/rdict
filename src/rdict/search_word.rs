extern crate pager_rs;
use crate::rdict::{
    format::{panel, wrap_text},
    structs::{Row, WordInfo},
};
use ansi_term::Style;
use crossterm::event::KeyCode;
use pager_rs::{Command, CommandList, CommandType, State, StatusBar};
use rusqlite::Connection;
use std::{error::Error, fmt::Write, process};

pub fn search_word(query_word: &String) {
    let mut output = String::new();
    match sql_query(query_word) {
        Ok(rows) => {
            for row in rows {
                if let Err(err) = print_word_information(&mut output, &row) {
                    eprintln!("{}", err);
                    process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            process::exit(1);
        }
    }

    let mut state = State::new(
        output,
        StatusBar::new("rdict".to_string()),
        CommandList::combine(vec![
            CommandList::default(),
            CommandList(vec![
                Command {
                    cmd: vec![CommandType::Key(KeyCode::Char('j'))],
                    desc: "Cursor down".to_string(),
                    func: &|state: &mut State| state.down(),
                },
                Command {
                    cmd: vec![CommandType::Key(KeyCode::Char('k'))],
                    desc: "Cursor up".to_string(),
                    func: &|state: &mut State| state.up(),
                },
            ]),
        ]),
    )
    .unwrap();
    state.show_line_numbers = false;
    pager_rs::init().unwrap();
    pager_rs::run(&mut state).unwrap();
    pager_rs::finish().unwrap();
}

// Function to format and print word information
fn print_word_information(output: &mut String, row: &Row) -> Result<(), Box<dyn Error>> {
    // Print Panel (Part of speech + Word)
    write!(
        output,
        "{}\n",
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
                writeln!(output, "{}", wrap_text(etymology, 2))?;
            }
        }
        // Print Definitions
        write!(output, "\n{}\n", Style::new().bold().paint("# Definitions"))?;
        for definition in &information.senses {
            for def in definition.glosses.as_ref().unwrap() {
                write!(
                    output,
                    "\n{}\n",
                    Style::new().bold().paint(wrap_text(def, 2))
                )?;
            }
            if let Some(examples) = &definition.examples {
                for example in examples {
                    if let Some(text) = &example.text {
                        write!(output, "\n{}\n", wrap_text(&format!("\"{}\"", text), 6))?;
                    }
                }
            }
        }
    }
    writeln!(output)?;
    Ok(())
}

// Function to execute the SQL query and retrieve word information
fn sql_query(query_word: &String) -> Result<Vec<Row>, Box<dyn Error>> {
    let path_to_db = crate::rdict::utils::get_home_directory()?.join(".local/share/rdict/en.db");
    let conn = Connection::open(path_to_db)?;
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
        return Err("No results".into());
    }
    Ok(results)
}

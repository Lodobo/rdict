use indicatif::ProgressBar;
use rdict::{
    error::AppError,
    structs::{Word, WordInfo},
    utils,
};
use rusqlite::{Connection, Result};
use std::{
    borrow::Cow,
    fs,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), AppError> {
    let reader = read_jsonl_file("en.jsonl")?;
    let mut conn = setup_database()?;
    let line_count = reader.lines().count() as u64;
    let progress_bar = ProgressBar::new(line_count);
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][ETA: {eta}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    let reader = read_jsonl_file("en.jsonl")?;
    process_lines(&mut conn, reader, &progress_bar)?;

    Ok(())
}

// Function to open the database and create the table
fn setup_database() -> Result<Connection, AppError> {
    let rdict_dir = utils::get_home_directory()?.join(".local/share/rdict");
    fs::create_dir_all(&rdict_dir)?;
    let conn = Connection::open(&rdict_dir.join("en.db"))?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS en (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            pos TEXT NOT NULL,
            information TEXT NOT NULL
         )",
        (),
    )?;
    Ok(conn)
}

// Function to open and read the JSONL file
fn read_jsonl_file(filename: &str) -> Result<BufReader<fs::File>, AppError> {
    let rdict_dir = utils::get_home_directory()?.join(".local/share/rdict");
    let file = fs::File::open(rdict_dir.join(filename))?;
    Ok(BufReader::new(file))
}

// Function to process each line of the JSONL file and insert into the database
fn process_lines(conn: &mut Connection, reader: BufReader<fs::File>, progress_bar: &ProgressBar) -> Result<(), AppError> {
    let transaction = conn.transaction()?;
    let reader = reader.lines();

    for (index, line) in reader.enumerate() {
        let line = line?;
        let word: Word = serde_json::from_str(&line)?;
        let information: WordInfo = serde_json::from_str(&line)?;
        let information: String = serde_json::to_string(&information)?;
        let information_cow: Cow<str> = Cow::Borrowed(&information);

        transaction.execute(
            "INSERT INTO en (word, pos, information) values (?1, ?2, ?3)",
            [&word.word, &word.pos, &information_cow],
        )?;

        progress_bar.set_position((index + 1000) as u64);
    }

    transaction.commit()?;
    progress_bar.finish_and_clear();
    Ok(())
}
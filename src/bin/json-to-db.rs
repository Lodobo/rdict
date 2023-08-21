use indicatif::ProgressBar;
use rdict::{
    structs::{Word, WordInfo},
    utils, error::AppError,
};
use rusqlite::{Connection, Result};
use std::{
    borrow::Cow,
    fs,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), AppError> {
    // Open directory where databse and json file is stored
    let rdict_dir = utils::get_home_directory()?.join(".local/share/rdict");
    // Open database
    let mut conn = Connection::open(&rdict_dir.join("en.db"))?;
    conn.execute(
        "create table if not exists en (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            pos TEXT NOT NULL,
            information TEXT NOT NULL
         )",
        (),
    )?;

    // Open JSON File
    let reader = BufReader::new(fs::File::open(rdict_dir.join("en.jsonl"))?);
    let line_count = reader.lines().count() as u64;
    let progress_bar = ProgressBar::new(line_count);
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][ETA: {eta}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    // Begin a transaction
    let transaction = conn.transaction()?;
    let reader = BufReader::new(fs::File::open(rdict_dir.join("en.jsonl"))?);
    for (index, line) in reader.lines().enumerate() {
        // deserialize, reorganize and reserialize.
        let word: Word = serde_json::from_str::<Word>(line.as_ref().unwrap())?;
        let information: WordInfo = serde_json::from_str::<WordInfo>(line.as_ref().unwrap())?;
        let information: String = serde_json::to_string(&information).unwrap();
        let information_cow: Cow<str> = Cow::Borrowed(&information);

        // Queue the INSERT statement within the transaction
        transaction.execute(
            "INSERT INTO en (word, pos, information) values (?1, ?2, ?3)",
            [&word.word, &word.pos, &information_cow],
        )?;
        progress_bar.set_position((index + 1000) as u64);
    }

    // Commit the transaction
    transaction.commit()?;
    progress_bar.finish_and_clear();

    Ok(())
}
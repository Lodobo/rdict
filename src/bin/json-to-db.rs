use indicatif::ProgressBar;
use rdict::{
    structs::{WordInfo, Word},
    utils,
};
use rusqlite::{Connection, Result};
use std::{
    fs,
    io::{BufRead, BufReader},
    borrow::Cow,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rdict_dir = utils::get_home_directory()?.join(".local/share/rdict");
    fs::create_dir_all(&rdict_dir)?;
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

    let file = fs::File::open(rdict_dir.join("en.jsonl"))?;
    let reader = BufReader::new(file);

    // Begin a transaction
    let transaction = conn.transaction()?;
    let line_count = reader.lines().count() as u64;
    let progress_bar = ProgressBar::new(line_count);
    progress_bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][ETA: {eta}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let file = fs::File::open(rdict_dir.join("en.jsonl"))?; // Open the file again
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let word: Word = serde_json::from_str::<Word>(line.as_ref().unwrap())?;
        let information: WordInfo = serde_json::from_str::<WordInfo>(line.as_ref().unwrap())?;
        let information: String = serde_json::to_string(&information).unwrap();
        let information_cow: Cow<str> = Cow::Borrowed(&information);

        // Queue the INSERT statement within the transaction
        transaction.execute(
            "INSERT INTO en (word, pos, information) values (?1, ?2, ?3)",
            [&word.word, &word.pos, &information_cow],
        )?;

        progress_bar.set_position((index + 1) as u64);
    }

    // Commit the transaction
    transaction.commit()?;
    progress_bar.finish_and_clear();

    Ok(())
}

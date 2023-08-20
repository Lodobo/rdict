use indicatif::ProgressBar;
use rdict::{structs, utils};
use rusqlite::{Connection, Result};
use std::{
    fs,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = utils::get_home_directory()?;
    let rdict_dir = home_dir.join(".local/share/rdict");
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
    let pb = ProgressBar::new(line_count);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][ETA: {eta}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let file = fs::File::open(rdict_dir.join("en.jsonl"))?; // Open the file again
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let word: structs::Word = serde_json::from_str::<structs::Word>(line.as_ref().unwrap())?;
        let information: structs::Information =
            serde_json::from_str::<structs::Information>(line.as_ref().unwrap())?;
        let information: String = serde_json::to_string(&information).unwrap();

        // Queue the INSERT statement within the transaction
        transaction.execute(
            "INSERT INTO en (word, pos, information) values (?1, ?2, ?3)",
            [&word.word, &word.pos, &information],
        )?;

        pb.set_position((index + 1) as u64);
    }

    // Commit the transaction
    transaction.commit()?;
    pb.finish_and_clear();

    Ok(())
}

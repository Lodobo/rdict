use crate::rdict::{
    structs::{Word, WordInfo},
    utils,
};
use indicatif::ProgressBar;
use rusqlite::{Connection, Result};
use std::{
    borrow::Cow,
    error::Error,
    fs,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn json_to_db() {
    match utils::get_home_directory() {
        Ok(home_dir) => {
            let rdict_dir = home_dir.join(".local/share/rdict");
            let path_to_json = rdict_dir.join("en.jsonl");
            match setup_database(&rdict_dir) {
                Ok(mut conn) => {
                    create_database_table(&mut conn);
                    process_lines(&mut conn, path_to_json)
                }
                Err(err) => eprintln!("Error: {}", err),
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn setup_database(rdict_dir: &std::path::PathBuf) -> Result<Connection, Box<dyn Error>> {
    fs::create_dir_all(rdict_dir)?;
    // Delete the existing database file if it exists
    let db_path = rdict_dir.join("en.db");
    if db_path.exists() {
        fs::remove_file(&db_path)?;
    }
    let conn = Connection::open(db_path)?;
    Ok(conn)
}

fn create_database_table(conn: &mut Connection) {
    if let Err(err) = conn.execute(
        "CREATE TABLE IF NOT EXISTS en (id INTEGER PRIMARY KEY AUTOINCREMENT, word TEXT NOT NULL, pos TEXT NOT NULL, information TEXT NOT NULL)",
        ()) {
            eprintln!("Error: {}", err);
    }
    if let Err(err) = conn.execute("CREATE INDEX IF NOT EXISTS idx_word ON en (word)", ()) {
        eprintln!("Error: {}", err);
    }
}

fn process_lines(conn: &mut Connection, path_to_json: PathBuf) {
    match fs::metadata(&path_to_json) {
        Ok(metadata) => {
            let file_size = metadata.len();
            let pb = ProgressBar::new(file_size);
            pb.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][ETA: {eta}]")
                    .unwrap()
                    .progress_chars("##-"),
            );
            let transaction: rusqlite::Transaction<'_> = conn.transaction().unwrap();
            let file = fs::File::open(path_to_json).unwrap();
            let reader = BufReader::new(file);

            for line in reader.lines() {
                // deserialize, reorganize and reserialize
                let word: Word = serde_json::from_str::<Word>(line.as_ref().unwrap()).unwrap();
                let word_info: WordInfo =
                    serde_json::from_str::<WordInfo>(line.as_ref().unwrap()).unwrap();
                let information: String = serde_json::to_string(&word_info).unwrap();
                let information_cow: Cow<str> = Cow::Borrowed(&information);
                if let Err(err) = transaction.execute(
                    "INSERT INTO en (word, pos, information) values (?1, ?2, ?3)",
                    [&word.word, &word.pos, &information_cow],
                ) {
                    eprintln!("Error: {}", err)
                }
                let byte_count = line.unwrap().len() as u64;
                pb.inc(byte_count);
            }

            if let Err(err) = transaction.commit() {
                eprintln!("Error: {}", err)
            }
            pb.finish_and_clear();
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

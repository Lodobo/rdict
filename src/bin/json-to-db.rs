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
    let rdict_dir = utils::get_home_directory()?.join(".local/share/rdict");
    let path_to_json = rdict_dir.join("en.jsonl");
    let mut conn = setup_database(&rdict_dir)?;
    create_database_table(&mut conn)?;
    if let Ok(pb) = create_pb(&path_to_json) {
        process_lines(&mut conn, &pb)?;
    }

    Ok(())
}

fn setup_database(rdict_dir: &std::path::PathBuf) -> Result<Connection, AppError> {
    fs::create_dir_all(rdict_dir)?;
    let conn = Connection::open(rdict_dir.join("en.db"))?;
    Ok(conn)
}

fn create_database_table(conn: &mut Connection) -> Result<(), AppError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS en (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word TEXT NOT NULL,
            pos TEXT NOT NULL,
            information TEXT NOT NULL
         )",
        (),
    )?;

    // Create an index on the "word" column
    conn.execute("CREATE INDEX IF NOT EXISTS idx_word ON en (word)", ())?;

    Ok(())
}

fn create_pb(path_to_json: &std::path::PathBuf) -> Result<ProgressBar, AppError> {
    let file_size = fs::metadata(path_to_json).unwrap().len();
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][ETA: {eta}]")?
            .progress_chars("##-"),
    );
    Ok(pb)
}

fn process_lines(conn: &mut Connection, pb: &ProgressBar) -> Result<(), AppError> {
    let path_to_json = utils::get_home_directory()?.join(".local/share/rdict/en.jsonl");
    let transaction: rusqlite::Transaction<'_> = conn.transaction()?;
    let file = fs::File::open(path_to_json)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // deserialize, reorganize and reserialize
        let word: Word = serde_json::from_str::<Word>(line.as_ref().unwrap())?;
        let word_info: WordInfo = serde_json::from_str::<WordInfo>(line.as_ref().unwrap())?;
        let information: String = serde_json::to_string(&word_info)?;
        let information_cow: Cow<str> = Cow::Borrowed(&information);
        transaction.execute(
            "INSERT INTO en (word, pos, information) values (?1, ?2, ?3)",
            [&word.word, &word.pos, &information_cow],
        )?;
        let byte_count = line.unwrap().len() as u64;
        pb.inc(byte_count);
    }

    transaction.commit()?;
    pb.finish_and_clear();

    Ok(())
}

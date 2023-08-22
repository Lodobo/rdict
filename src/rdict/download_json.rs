use crate::rdict::error::AppError;
use indicatif::ProgressBar;
use std::{
    fs,
    io::{Read, Write},
};

pub fn download_json() {
    let url = "https://kaikki.org/dictionary/English/kaikki.org-dictionary-English.json";
    match download_and_save(url) {
        Ok(_) => println!("Dictionary downloaded successfully as: en.jsonl"),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn download_and_save(url: &str) -> Result<(), AppError> {
    let mut response = reqwest::blocking::get(url)?;
    let total_size = response.content_length().unwrap_or(0);
    let progress_bar = ProgressBar::new(total_size);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(
                "[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][{bytes_per_sec}][ETA: {eta}]",
            )?
            .progress_chars("##-"),
    );

    let rdict_dir = crate::rdict::utils::get_home_directory()?.join(".local/share/rdict");
    fs::create_dir_all(&rdict_dir)?;
    let mut file = fs::File::create(&rdict_dir.join("en.jsonl"))?;
    let mut buffer = [0; 8192];
    let mut downloaded = 0;

    println!("Downloading dictionary from {}", url);
    println!("Saving to: {}", &rdict_dir.join("en.jsonl").display());

    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        progress_bar.set_position(downloaded);
    }

    progress_bar.finish_with_message("Download complete.");

    Ok(())
}

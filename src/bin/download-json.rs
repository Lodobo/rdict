use indicatif::ProgressBar;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://kaikki.org/dictionary/English/kaikki.org-dictionary-English.json";

    match download_file(url) {
        Ok(_) => println!("File downloaded successfully as: en.jsonl"),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}

fn download_file(url: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let total_size = response.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "[{bar:40.cyan/blue}][{percent}%][{elapsed_precise}][{bytes_per_sec}][ETA: {eta}]",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    let home_dir = rdict::utils::get_home_directory()?;
    let rdict_dir = home_dir.join(".local/share/rdict");
    fs::create_dir_all(&rdict_dir)?;
    let mut file = fs::File::create(&rdict_dir.join("en.jsonl"))?;
    let mut buffer = [0; 8192];
    let mut downloaded = 0;

    println!("Downloading {}", url);
    println!("Path: {}", &rdict_dir.join("en.jsonl").display());

    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete.");

    Ok(())
}

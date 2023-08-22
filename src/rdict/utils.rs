use std::{error::Error, path::PathBuf};
pub fn get_home_directory() -> Result<PathBuf, Box<dyn Error>> {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            return Err("Failed to retrieve home directory".into());
        }
    };
    Ok(home_dir)
}

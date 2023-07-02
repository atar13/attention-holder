use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

// will pull from google drive and save file to specified path
// requires share_id from google drive sharing link
pub fn pull_from_drive(share_id: &str, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get(format!(
        "https://docs.google.com/uc?export=download&id={}",
        share_id
    ))?;
    let body = resp.bytes()?;
    let mut file = File::create(path)?;
    file.write_all(&body)?;
    Ok(())
}

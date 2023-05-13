mod common;

use std::{error::Error, env};

use attention_holder::pdf::PDF;

#[test]
fn open_pdf() -> Result<(), Box<dyn Error>> {
    let pdf_path = env::temp_dir().join("presentation.pdf");
    common::pull_from_drive("1EpINFh3wE8b2fsCsVAcXhpi8_Yt_yzH3", &pdf_path)?;

    let pdf = PDF::from_path(pdf_path.to_str().unwrap());

    assert_eq!(pdf.pages.len(), 2);
    assert_eq!(pdf.title, "Untitled presentation");

    Ok(())
}

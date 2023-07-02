pub mod html;
pub mod pdf;

use std::io;
use std::io::ErrorKind::AlreadyExists;
use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;

use crate::pdf::PDF;

#[derive(Parser)]
#[command(version, about = "Hold your audience's attention during presentations", long_about = None)]
pub struct Config {
    // path to pdf file with actual content
    pdf_path: String,
    // video to display on the side
    zoomer_video: String,

    #[arg(short, long, help = "Name of the folder to output to", default_value_t = String::from("output"))]
    output: String,

    #[arg(
        short,
        long,
        help = "Disables any audio that may be in the videos",
        default_value_t = false
    )]
    no_audio: bool,
}

fn main() {
    let config = Config::parse();

    let pdf = PDF::from_path(&config.pdf_path);

    let output_dir = Path::new(&config.output);
    let res = fs::create_dir(&output_dir);
    ignore_dir_already_exists(res, output_dir);

    let assets_dir = output_dir.join("assets");
    let res = fs::create_dir(&assets_dir);
    ignore_dir_already_exists(res, output_dir);

    let vid = PathBuf::from(&config.zoomer_video);
    let new_vid_path = assets_dir.join(vid.file_name().unwrap());
    fs::copy(&vid, &new_vid_path).unwrap();

    let slide_imgs = pdf.save_pages(&output_dir);

    let mut relative_vid_path = PathBuf::new();
    for (idx, component) in new_vid_path.iter().enumerate() {
        if idx == 0 {
            continue;
        }
        relative_vid_path = relative_vid_path.join(component);
    }

    let document = html::generate_html(
        pdf.title.to_owned(),
        slide_imgs,
        &relative_vid_path,
        &config,
    );
    document.save_to_file(&output_dir.join(format!("{}.html", pdf.title)));
}

// ignore error if folder already exists
fn ignore_dir_already_exists(res: io::Result<()>, dir: &Path) {
    if let Err(e) = res {
        match e.kind() {
            AlreadyExists => (),
            _ => panic!("Cannot create directory: {}", dir.display()),
        }
    }
}

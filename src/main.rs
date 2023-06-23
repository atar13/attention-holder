pub mod pdf;
pub mod html;

use std::{path::PathBuf, fs};

use clap::{Parser, ValueEnum};

use crate::pdf::PDF;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Parser)]
#[command(version, about = "Hold your audience's attention during presentations", long_about = None)]
struct Config {
    // path to pdf file with actual content
    pdf_path: String,
    // list of videos to display on the side
    zoomer_videos: Vec<String>,

    #[arg(short, long, help = "Name of the folder to output to", default_value_t = String::from("output"))]
    output: String,

    #[arg(
        long,
        help = "Use the videos in the order given, otherwise order is randomized",
        default_value_t = false
    )]
    ordered: bool,

    #[arg(
        short,
        long,
        help = "Number of zoomer videos to include in each slide",
        default_value_t = 1
    )]
    zoomer_level: u8,

    #[arg(short, long, help = "Percentage of the screen the actual content should take up", default_value_t = 50, value_parser = clap::value_parser!(u8).range(1..100))]
    content_percentage: u8,

    #[arg(
        short,
        long,
        help = "The amount of time the presentation should wait on each slide before automatically advancing. 0 will not adance automatically",
        default_value_t = 0
    )]
    slide_time: u8,

    #[arg(short, long, help = "Specifies the direction that the presentation should flow", value_enum, default_value_t = Direction::DOWN)]
    direction: Direction,

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

    let pdf = PDF::from_path(config.pdf_path.as_str());

    let output_dir = PathBuf::from(config.output);

    let mut vid_relative_paths: Vec<PathBuf> = Vec::new();
    let vid_pathbufs: Vec<PathBuf> = config.zoomer_videos.iter().map(|x| PathBuf::from(x)).collect();
    for vid in &vid_pathbufs {
        let file_name = vid.file_name().unwrap();
        fs::copy(vid, &output_dir.join(file_name)).unwrap();
        vid_relative_paths.push(file_name.into());
    }

    let slide_imgs = pdf.save_pages(&output_dir);
    let html = html::generate_html(pdf.title.to_owned(), slide_imgs, vid_relative_paths);
    html.save_to_file(&output_dir.join(format!("{}.html", pdf.title)));
}

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};

use build_html::{Container, ContainerType, Html, HtmlContainer, HtmlPage};

use crate::Config;

pub struct HTML {
    content: String,
}

// generate html document
pub fn generate_html(
    document_title: String,
    slide_imgs: Vec<PathBuf>,
    zoomer_vid: &Path,
    config: &Config,
) -> HTML {
    HTML {
        content: HtmlPage::new()
            .with_meta(vec![("charset", "utf-8")])
            .with_meta(vec![
                ("http-equiv", "X-UA-Compatible"),
                ("content", "IE=edge"),
            ])
            .with_meta(vec![
                ("name", "viewport"),
                ("content", "width=device-width"),
                ("initial-scale", "1.0"),
            ])
            .with_title(document_title)
            .with_style(include_str!("style.css"))
            .with_script_literal(include_str!("script.js"))
            .with_container(
                Container::new(ContainerType::Main)
                    // slides container
                    .with_container(
                        slide_imgs.iter().enumerate().fold(
                            Container::new(ContainerType::Div)
                                .with_attributes(vec![("id", "presentation")]),
                            |acc, (idx, img)| {
                                acc.with_image_attr(
                                    img.to_string_lossy(),
                                    img.file_name()
                                        .unwrap_or(OsStr::new("slide"))
                                        .to_string_lossy(),
                                    vec![
                                        ("id", format!("slide-{idx}").as_str()),
                                        ("class", "slide"),
                                    ],
                                )
                            },
                        ),
                    )
                    // resizer container
                    .with_container(
                        Container::new(ContainerType::Div).with_attributes(vec![("id", "resizer")]),
                    )
                    // video container
                    .with_container(
                        Container::new(ContainerType::Div)
                            .with_attributes(vec![("id", "zoomer-vid"), ("style", "flex: 1 1 0%")])
                            .with_html(Video::new(
                                zoomer_vid.to_string_lossy(),
                                true,
                                true,
                                config.no_audio,
                            )),
                    ),
            )
            .to_html_string(),
    }
}

impl HTML {
    // save html document to a file at a given path
    pub fn save_to_file(&self, path: &PathBuf) {
        let mut file = File::create(path).expect(&format!(
            "Could not create html file at {}",
            path.to_string_lossy()
        ));
        file.write_all(self.content.as_bytes())
            .expect(&format!("Could not write to {}", path.to_string_lossy()));
        println!("Saved to {:?}", path);
    }
}

#[derive(Debug)]
struct Video {
    src: String,
    loops: bool,
    autoplay: bool,
    muted: bool,
}

impl Video {
    pub fn new(src: impl ToString, loops: bool, autoplay: bool, muted: bool) -> Self {
        Video {
            src: src.to_string(),
            loops,
            autoplay,
            muted,
        }
    }
}

impl Html for Video {
    fn to_html_string(&self) -> String {
        let loop_str = if self.loops { "loop" } else { "" };
        let autoplay_str = if self.autoplay { "autoplay" } else { "" };
        let muted_str = if self.muted { "muted" } else { "" };
        format!(
            "<video {} {} {} src={}></video>",
            loop_str, autoplay_str, muted_str, self.src
        )
    }
}

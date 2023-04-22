use std::{
    fs::File,
    path::Path,
};

use cairo::{Context, Format, ImageSurface};
use poppler::{Document, Page};

pub struct PDF {
    pub title: String,
    document: Document,
    pages: Vec<Page>,
}

impl PDF {
    pub fn from_path(path: &str) -> PDF {
        let abs_pdf_path = Path::canonicalize(Path::new(path))
            .expect(format!("Could not access {}.", path,).as_str());
        let document =
            Document::from_file(format!("file://{}", abs_pdf_path.display()).as_str(), None)
                .expect(format!("Could not open {} as input PDF.", path).as_str());
        let title = match document.title() {
            Some(gstring) => gstring.as_str().to_owned(),
            None => "Untitled".to_owned(),
        };
        let pages = (0..document.n_pages()).fold(Vec::new(), |mut acc, i| {
            acc.push(document.page(i).unwrap());
            acc
        });
        // let permissions = doc.permissions();

        PDF {
            title,
            document,
            pages,
        }
    }

    pub fn save_pages(&self, output_dir: &Path) {
        // TODO: add actual error handling
        if !output_dir.exists() {
            panic!("Must create output directory") }
        if !output_dir.is_dir() {
            panic!("Must save pages to a directory")
        }

        for (i, page) in self.pages.iter().enumerate() {
            save_page_to_png(
                &page,
                output_dir
                    .join(Path::new(&format!("slide_{}.png", i + 1)))
                    .as_path(),
            )
        }
    }
}

fn save_page_to_png(page: &Page, path: &Path) {
    // TODO: add actual error handling
    match path.extension() {
        Some(ext) if ext == "png" => (),
        _ => panic!("Can't save page to a non PNG file"),
    };

    let (w, h) = page.size();
    let surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).unwrap();
    let ctx = Context::new(&surface).unwrap();

    ctx.save().unwrap();
    page.render(&ctx);
    ctx.restore().unwrap();
    ctx.show_page().unwrap();

    let mut f: File = File::create(path).unwrap();
    surface.write_to_png(&mut f).expect("Unable to write PNG");
}
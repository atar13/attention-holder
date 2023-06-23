use std::ffi::OsStr;
use std::{io::Write, fmt::Display};
use std::path::PathBuf;
use std::fs::File;

use html::metadata::Head;
use html::scripting;

use build_html::{HtmlPage, Html, HtmlContainer, Container, ContainerType};


#[derive(Debug)]
struct Video {
    src: String,
    loops: bool
}

impl Video {
    pub fn new(src: impl ToString, loops: bool) -> Self {
        Video { src: src.to_string(), loops }
    }

}


impl Html for Video {
    fn to_html_string(&self) -> String {
        if self.loops {
            format!("<video loop src={}></video>", self.src)
        } else {
            format!("<video src={}></video>", self.src)
        }
    }
}

pub struct HTML {
    content: String
}


pub fn generate_html(document_title: String, slide_imgs: Vec<PathBuf>, zoomer_vids: Vec<PathBuf>) -> HTML {
    // let script = scripting::Script::builder().text("hi").build();
    // let head = Head::builder().
    //     meta(|x| x.charset("UTF-8")).
    //     meta(|x| x.http_equiv("X-UA-Compatible").content("IE=edge")).
    //     meta(|x| x.name("viewport").content("width=device-width, initial-scale=1.0")).
    //     title(|x| x.text(document_title)).
    //     style(|x| x.text("/* CSS goes here */")).
    //     push(script).
    //     build();
    // println!("html:\n{}", head.to_string());

    // let page = HtmlPage::new().
    //     with_meta(vec![("charset", "utf-8")]).
    //     with_meta(vec![("http-equiv", "X-UA-Compatible"), ("content", "IE=edge")]).
    //     with_meta(vec![("name", "viewport"), ("content", "width=device-width"), ("initial-scale", "1.0")]).
    //     with_title(document_title).
    //     with_style(include_str!("style.css")).
    //     with_script_literal(include_str!("script.js")).
    //     with_container(
    //         slide_imgs.iter().enumerate().fold(
    //             Container::new(ContainerType::Main), 
    //             |acc, (idx, img)| acc.with_container(
    //                 Container::new(ContainerType::Section).with_attributes(vec![("id", format!("slide-{idx}").as_str())]).
    //                 with_container(Container::new(ContainerType::Div).with_attributes(vec![("class", "presentation")]).
    //                     with_image(img.to_string_lossy(), img.file_name().unwrap_or(OsStr::new("slide")).to_string_lossy())
    //                 ).
    //                 with_container(
    //                     zoomer_vids.iter().fold(
    //                         Container::new(ContainerType::Div).with_attributes(vec![("class", "zoomer-vid")]), 
    //                         |acc, vid| acc.with_html(Video::new(vid.to_string_lossy(), true))
    //                     )
    //                 )
    //             )
    //         )
    //     );
    let page = HtmlPage::new().
        with_meta(vec![("charset", "utf-8")]).
        with_meta(vec![("http-equiv", "X-UA-Compatible"), ("content", "IE=edge")]).
        with_meta(vec![("name", "viewport"), ("content", "width=device-width"), ("initial-scale", "1.0")]).
        with_title(document_title).
        with_style(include_str!("style.css")).
        with_script_literal(include_str!("script.js")).
        with_container(Container::new(ContainerType::Main).
            with_container(
                slide_imgs.iter().enumerate().fold(
                    Container::new(ContainerType::Div).with_attributes(vec![("id", "presentation")]), 
                    |acc, (idx, img)| acc.
                        with_image_attr(img.to_string_lossy(), img.file_name().unwrap_or(OsStr::new("slide")).to_string_lossy(), vec![("id", format!("slide-{idx}").as_str()), ("class", "slide")])
                )
            ).
            with_container(
                zoomer_vids.iter().fold(
                    Container::new(ContainerType::Div).with_attributes(vec![("id", "zoomer-vid")]), 
                    |acc, vid| acc.with_html(Video::new(vid.to_string_lossy(), true))
                )
            )
        );


    HTML { content: page.to_html_string() }
    // page.to_html_string()
}

impl HTML {
    pub fn save_to_file(&self, path: &PathBuf) {
        let mut file = File::create(path).expect(&format!("Could not create html file at {}", path.to_string_lossy()));
        file.write_all(self.content.as_bytes()).expect(&format!("Could not write to {}", path.to_string_lossy()));
        println!("Saved to {:?}", path);
    }
}

fn generate_head() -> String {
    format!("<head>
        <meta charset=\"UTF-8\">
        <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Document</title>
        <style>
            /* CSS RESET */
            html,body,div,p,h1,h2,h3,h4,h5,h6,ul,ol,li,dl,dt,dd,form,fieldset,caption,table,tr,td,th,address,blockquote,img {{
                margin:0;
                padding:0;
            }}

            img, fieldset, object {{
                border:none;
            }}

            *, *:after, *:before {{
                flex:1 0 auto; /* safari bugfix */
                box-sizing:border-box;
            }}

            button, label {{
                cursor:pointer;
            }}

            html, body {{
                min-height:100%;
            }}

            body {{
                overflow: hidden;
            }}

            /* OUR STYLING */

            main {{
                display: flex;
                flex-direction: column;
            }}

            img {{
                object-fit: fill;
                display: block;
                width: 100%;
                height: 100%;
            }}

            video {{
                object-fit: fill;
            }}

            section {{
                width: 100%;
                height: 100vh;
                display: flex;
            }}

            section aside {{
                display: flex;
            }}

            #controls {{
                z-index: 1000;
                position: absolute;
            }}

        </style>
        <script defer>
            let index = 0;
            let oldIndex = 0;

            const NUM_VIDEOS = 2;
            const CONTENT_PERCENTAGE = 0.60;
            const START_TIME_RANDOMIZE = true;

            // still need to generalize based on 

            // SLIDE_TIME
            // DIRECTION
            // NO AUDIO

            function pauseSection(section) {{
                for (const video of section.querySelectorAll('video')) {{
                    video.pause();
                }}
            }}

            function playSection(section) {{
                for (const video of section.querySelectorAll('video')) {{
                    video.play();
                    if (START_TIME_RANDOMIZE) {{
                        video.currentTime = video.duration * Math.random();
                    }}
                }}
            }}

            window.addEventListener(\"keydown\", (e) => {{
                e.preventDefault();
                switch (e.key) {{
                    case \"ArrowDown\":
                        index+=1
                        break;
                    case \"ArrowUp\":
                        index-=1;
                        break;
                }}

                let size = document.getElementsByTagName('main')[0].children.length;
                if (index < 0) {{ index = 0; }}
                if (index >= size) {{ index = size - 1; }}

                if (index != oldIndex) {{
                    pauseSection(document.getElementById(`slide-${{oldIndex}}`));
                    oldIndex = index;
                    document.getElementById(`slide-${{index}}`).scrollIntoView({{behavior: \"smooth\"}});
                    playSection(document.getElementById(`slide-${{index}}`));
                }}
            }});

            document.addEventListener(\"DOMContentLoaded\", () => {{
                function setupSlides() {{
                    let width = window.innerWidth
                    || document.documentElement.clientWidth
                    || document.body.clientWidth;

                    let height = window.innerHeight
                    || document.documentElement.clientHeight
                    || document.body.clientHeight;

                    let sectionDir = \"\";
                    if (width > height) {{
                        sectionDir = \"row\";
                    }} else {{
                        sectionDir = \"column\";
                    }}

                    for (const section of document.querySelectorAll(\"section\")) {{
                        section.style.flexDirection = sectionDir;
                    }}


                    for (const div of document.querySelectorAll(\"div\")) {{
                        let img = div.querySelector(\"img\");
                        if (width > height) {{
                            div.style.width = `${{width * CONTENT_PERCENTAGE}}px`;
                            div.style.height = `${{height}}px`;
                            img.width = width * CONTENT_PERCENTAGE;
                        }} else {{
                            div.style.height = `${{height * CONTENT_PERCENTAGE}}`;
                            div.style.width = `${{width}}`;
                            img.height= width * CONTENT_PERCENTAGE;
                        }}
                    }}

                    let asideDir = \"\";
                    if (width > height) {{
                        asideDir = \"column\";
                    }} else {{
                        asideDir = \"row\";
                    }}

                    for (const aside of document.querySelectorAll(\"aside\")) {{
                        aside.style.flexDirection = asideDir;
                    }}

                    let videoPercentage = (1 - CONTENT_PERCENTAGE) / NUM_VIDEOS;

                    for (const video of document.querySelectorAll(\"video\")) {{
                        if (width > height) {{
                            video.width = width * (1-CONTENT_PERCENTAGE);
                            video.height = height * videoPercentage;
                        }} else {{
                            video.height = height * (1-CONTENT_PERCENTAGE);
                            video.width = height * videoPercentage;
                        }}
                    }}
                }}

                setupSlides();
                window.addEventListener(\"resize\", setupSlides);

                window.addEventListener(\"mousemove\", (event) => {{
                    const controls = getElementById(\"controls\");
                    console.log(controls.style.color);
                    console.log(controls.style.background-color);
                    controls.style.background-color = \"blue\";
                }});
            }});
            
        </script>

    </head>\n")
}

fn generate_body(slide_imgs: Vec<PathBuf>, zoomer_vids: Vec<PathBuf>) -> String {
    let mut body = String::from("<body>\n\
                                <main>\n");
    body = body + &format!("<div id=\"controls\">
                            <h1>
                                Button
                            </h1>
                           </div>
                          ");
    body = body + &slide_imgs.iter().enumerate().fold("".to_owned(), 
                                         |acc, (idx, slide)| acc + &generate_page(slide, idx, &zoomer_vids)
                                         );

    body = body + "</main>\n\
                    <body>\n";
    body
}

fn generate_page(slide_img: &PathBuf, slide_idx: usize, zoomer_vids: &Vec<PathBuf>) -> String {

    let mut page = format!("<section id=\"slide-{}\">\n\t<div><img src=\"{}\"></img></div>\n", slide_idx, slide_img.to_string_lossy());
    page = page + "\t<aside>";
    page = page + &zoomer_vids.iter().fold("".to_owned(), |acc, vid| 
                                         acc + &format!("<video src=\"{}\" ></video>", vid.to_string_lossy()));
    page = page + "</aside>\n";
    page = page + "</section>\n";
    page
}


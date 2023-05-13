use std::io::Write;
use std::path::PathBuf;
use std::fs::File;



pub struct HTML {
    content: String
}

pub fn generate_html(slide_imgs: Vec<PathBuf>, zoomer_vids: Vec<PathBuf>) -> HTML {
    let mut html = String::from("<!DOCTYPE html>\n\
                            <html lang=\"en\">\n");
    html = html + &generate_head();
    html = html + &generate_body(slide_imgs, zoomer_vids);
    html = html + "</html>";

    HTML { content: html }
}

impl HTML {
    pub fn save_to_file(&self, path: &PathBuf) {
        let mut file = File::create(path).expect(&format!("Could not create html file at {}", path.to_string_lossy()));
        file.write_all(self.content.as_bytes()).expect(&format!("Could not write to {}", path.to_string_lossy()));
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
            }});
            
        </script>

    </head>")
}

fn generate_body(slide_imgs: Vec<PathBuf>, zoomer_vids: Vec<PathBuf>) -> String {
    let mut body = String::from("<body>\n\
                                <main>\n");
    body = body + &slide_imgs.iter().enumerate().fold("".to_owned(), 
                                         |acc, (idx, slide)| acc + &generate_page(slide, idx, &zoomer_vids)
                                         );

    body = body + "</main>\n\
                    <body>\n";
    body
}

fn generate_page(slide_img: &PathBuf, slide_idx: usize, zoomer_vids: &Vec<PathBuf>) -> String {

    let mut page = format!("<section id=\"slide-{}\">\n<div>\n<img src=\"{}\"></img>\n</div>", slide_idx, slide_img.to_string_lossy());
    page = page + "<aside>";
    page = page + &zoomer_vids.iter().fold("".to_owned(), |acc, vid| 
                                         acc + &format!("<video src=\"{}\" ></video>\n", vid.to_string_lossy()));
    page = page + "</aside>";
    page
}


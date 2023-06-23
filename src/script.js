let index = 0;
let oldIndex = 0;

const NUM_VIDEOS = 2;
const CONTENT_PERCENTAGE = 0.60;
const START_TIME_RANDOMIZE = true;

// still need to generalize based on 

// SLIDE_TIME
// DIRECTION
// NO AUDIO

function pauseSection(section) {
    for (const video of section.querySelectorAll('video')) {
        video.pause();
    }
}

function playSection(section) {
    for (const video of section.querySelectorAll('video')) {
        video.play();
        if (START_TIME_RANDOMIZE) {
            video.currentTime = video.duration * Math.random();
        }
    }
}

window.addEventListener("keydown", (e) => {
    e.preventDefault();
    switch (e.key) {
        case "ArrowDown":
            index+=1
            break;
        case "ArrowUp":
            index-=1;
            break;
    }

    let size = document.getElementById('presentation').children.length;
    if (index < 0) { index = 0; }
    if (index >= size) { index = size - 1; }

    if (index != oldIndex) {
        // pauseSection(document.getElementById(`slide-${oldIndex}`));
        // document.getElementById(`slide-${index}`).scrollIntoView({behavior: "smooth"});
        document.getElementById(`slide-${oldIndex}`).style.display = "none";
        document.getElementById(`slide-${index}`).style.display = "block";
        // playSection(document.getElementById(`slide-${index}`));
        oldIndex = index;
    }
});

document.addEventListener("DOMContentLoaded", () => {
    function setupSlides() {
        let width = window.innerWidth
        || document.documentElement.clientWidth
        || document.body.clientWidth;

        let height = window.innerHeight
        || document.documentElement.clientHeight
        || document.body.clientHeight;

        let sectionDir = "";
        if (width > height) {
            sectionDir = "row";
        } else {
            sectionDir = "column";
        }

        for (const section of document.querySelectorAll("section")) {
            section.style.flexDirection = sectionDir;
        }


        // for (const div of document.querySelectorAll("div")) {
        //     let img = div.querySelector("img");
        //     if (width > height) {
        //         div.style.width = `${width * CONTENT_PERCENTAGE}px`;
        //         div.style.height = `${height}px`;
        //         img.width = width * CONTENT_PERCENTAGE;
        //     } else {
        //         div.style.height = `${height * CONTENT_PERCENTAGE}`;
        //         div.style.width = `${width}`;
        //         img.height= width * CONTENT_PERCENTAGE;
        //     }
        // }

        // document.getElementById('slide-0').style.display = "block";

        // let asideDir = "";
        // if (width > height) {
        //     asideDir = "column";
        // } else {
        //     asideDir = "row";
        // }
        //
        // for (const aside of document.getElementsByClassName("zoomer-vid")) {
        //     aside.style.flexDirection = asideDir;
        // }
        //
        // let videoPercentage = (1 - CONTENT_PERCENTAGE) / NUM_VIDEOS;
        //
        // for (const video of document.querySelectorAll("video")) {
        //     if (width > height) {
        //         video.width = width * (1-CONTENT_PERCENTAGE);
        //         video.height = height * videoPercentage;
        //     } else {
        //         video.height = height * (1-CONTENT_PERCENTAGE);
        //         video.width = height * videoPercentage;
        //     }
        // }
    }

    setupSlides();
    window.addEventListener("resize", setupSlides);
});

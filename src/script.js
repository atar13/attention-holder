// presentation slide index
let index = 0;
// previous presentation slide index
let oldIndex = 0;

// current mouse positions
let mouse_x = 0;
let mouse_y = 0;

// keep track of presentation width as it's resized
let slidesWidth = 0;

// change slide by a given offset
function changeSlide(i) {
    index += i;
    let size = document.getElementById('presentation').children.length;
    if (index < 0) { index = 0; }
    if (index >= size) { index = size - 1; }

    if (index != oldIndex) {
        document.getElementById(`slide-${oldIndex}`).style.display = "none";
        document.getElementById(`slide-${index}`).style.display = "block";
        oldIndex = index;
    }
}

// listen for up/down keypresses to change slides
window.addEventListener("keydown", (e) => {
    e.preventDefault();
    switch (e.key) {
        case "ArrowDown":
            changeSlide(1);
            break;
        case "ArrowRight":
            changeSlide(1);
            break;
        case "ArrowUp":
            changeSlide(-1);
            break;
        case "ArrowLeft":
            changeSlide(-1);
            break;
    }
});

document.addEventListener("DOMContentLoaded", () => {
    const resizer = document.getElementById("resizer");
    const slidesElement = resizer.previousElementSibling;
    const videoElement = resizer.nextElementSibling;

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

        for (const main of document.querySelectorAll("main")) {
            main.style.flexDirection = sectionDir;
        }
    }

    // initial setup for resizing when mouse is clicked
    function startResize (e) {
        mouse_x = e.clientX;
        mouse_y = e.clientY;
        slidesWidth = slidesElement.getBoundingClientRect().width;

        document.addEventListener('mousemove', resize);
        document.addEventListener('mouseup', stopResize);
    }

    // resizes width of slides according to mouse position
    function resize(e) {
        // change in x position on mouse movement
        const dx = e.clientX - mouse_x;

        const newLeftWidth = ((slidesWidth + dx) * 100) / resizer.parentNode.getBoundingClientRect().width;
        slidesElement.style.width = `${newLeftWidth}%`;

        resizer.style.cursor = 'col-resize';
        document.body.style.cursor = 'col-resize';

        slidesElement.style.userSelect = 'none';
        slidesElement.style.pointerEvents = 'none';

        videoElement.style.userSelect = 'none';
        videoElement.style.pointerEvents = 'none';
    }

    // handle letting go of the mouse to stop resizing content
    function stopResize (e) {
        resizer.style.removeProperty('cursor');
        document.body.style.removeProperty('cursor');

        slidesElement.style.removeProperty('user-select');
        slidesElement.style.removeProperty('pointer-events');

        videoElement.style.removeProperty('user-select');
        videoElement.style.removeProperty('pointer-events');

        document.removeEventListener('mousemove', resize);
        document.removeEventListener('mouseup', stopResize);
    };

    function clickNextSlide (e) {
        // ignore event if we clicked on the resizer since that element is used to resize
        if (e.target.id == "resizer") {
            return
        }
        changeSlide(1);
    }


    setupSlides();
    window.addEventListener("resize", setupSlides);

    window.addEventListener("click", clickNextSlide);
    resizer.addEventListener('mousedown', startResize);
});

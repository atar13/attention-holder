# attention-holder

Hold your audience's attention with this one simple trick.

<!-- add example of running the command with an example output slideshow  -->

## Usage

### Standard 
```
attention-holder [pdf_file] [zoomer_video_1] [zoomer_video_2] ... [zoomer_video_n]
```

- `pdf_file`: presentation/document you want to make palatable to the younger generation
    - in the future we may support additional input types
- `zoomer_video_n`: video file to include within the presentation

### Optional Arguments

- `--output=[file_name]`
    - Name of the file to output to
    - Defaults to `zoomer.html`
    - The output format will be html, so you should probably use a .html file extension.
        - In the future we may support output to future formats
- `--ordered`
    - If specified, use the videos in the order given to the program. 
    - If not specified, then randomly place videos into the presentation.
- `--zoomer-level=[number]`
    - Number of zoomer videos to include in each slide.
    - Defaults to 1 if not specified.
- `--content-percentage=[number]`
    - Percentage of the screen that the actual content takes up.
    - The remaining screen peercentage will be taken up by the zoomer videos, split evenly amongst them.
    - Defaults to 50 if not specified.
- `--slide-time=[number]`
    - The amount of time the presentation should wait on each slide before automatically advancing
    - If not specified or set to 0, then the presentation will not automatically advance, and the presenter will have to manually move through the presentation.
- `--direction=[up/down/left/right]`
    - Specifies the direction that the presentation should flow
    - Defaults to down if not specified.
- `--no-audio`
    - Disables any audio that may be in the videos
    - For more fine tuned control on a video-by-video basis, you should edit the video file directly to remove the sound.
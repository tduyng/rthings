# tubed
`tubed` is a command-line tool for downloading and converting YouTube videos to various formats. It use the [yt-dlp](https://github.com/yt-dlp/yt-dlp) tool under the hood to provide powerful and flexible video downloading capabilities.

## Features
Download YouTube videos in various formats including MP4 and MP3.
Specify output directory for downloaded files.
Additional options to customize the download process.

## Requirements
- Linux / Mac - build with POSIX system paths in mind (Windows might work)
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) above 2023.03.03*1 and be accessable via the command `yt-dlp`
- [ffmpeg](https://ffmpeg.org/) is installed and be accessable via the command ffmpeg

## Installation
I haven't published the package to crates.io yet. If you'd like to use this tool, you can install it locally by following these steps:

### Clone the project and install

1. Clone the project repository to your local machine and navigate to the tubed directory
```bash
git clone https://github.com/tduyng/rthings.git
cd rthings/projects/tubed
```

2. Install the tool using Cargo
```bash
cargo install --path .
```

### Install from Remote URL
You can also install directly from the GitHub repository using the following command:

```bash
Cargo install --git https://github.com/tduyng/rthings/tree/main/projects/tubed
```


## Usage
To see all available command options, run the following `help` command:
```bash
tubed -h
```
Output:

```md
Downloads YouTube videos

Usage: tubed [OPTIONS] --url <URL>

Options:
      --url <URL>          The YouTube URL to download
  -f, --format <FORMAT>    Output format (e.g., mp4, mp3, best) [default: mp4]
  -o, --output <OUTPUT>    Output directory for the downloaded files [default: .]
      --options <OPTIONS>  Additional yt-dlp options
  -h, --help               Print help
  -V, --version            Print version
```

### Basic usage
To download a YouTube video with default settings (MP4 format) to the current directory, simply run:

```bash
tubed --url <VIDEO_URL>
```

Replace <VIDEO_URL> with the URL of the YouTube video you want to download.

## Specifying output format
You can specify the output format using the --format option. For example, to download a video in MP3 format, run:

```bash
tubed --url <VIDEO_URL> --format mp3
```

### Specifying output directory
To specify a custom output directory for the downloaded files, use the`--output` option. 

For example:

```bash
tubed --url <VIDEO_URL> --output /path/to/output/directory
```

### Additional options
`tubed` supports passing additional options to the underlying `yt-dlp` tool using the `--options` flag. 

For example:

```bash
tubed --url <VIDEO_URL> --options "--extract-audio --audio-format opus"
```

This command will extract the audio from the video and save it in Opus format.

## Example
### Command
```bash
tubed --url https://www.youtube.com/watch?v=o2ob8zkeq2s --format mp3 --output download
```

### Console output
```bash
[youtube] Extracting URL: https://www.youtube.com/watch?v=o2ob8zkeq2s
[youtube] o2ob8zkeq2s: Downloading webpage
[youtube] o2ob8zkeq2s: Downloading ios player API JSON
[youtube] o2ob8zkeq2s: Downloading m3u8 information
[info] o2ob8zkeq2s: Downloading 1 format(s): 251
[download] Destination: download/Example Title.webm
[download] 100% of   20.02MiB in 00:00:00 at 41.34MiB/s    
[ExtractAudio] Destination: download/Example Title.mp3
Deleting original file download/Example Title.webm (pass -k to keep)
Download and conversion completed successfully in 5.24 seconds.
```
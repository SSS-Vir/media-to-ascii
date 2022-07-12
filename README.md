# media-to-ascii

***~~Useless piece of s...~~ created at 3 am because... just because***

## Build from source
```sh
git clone https://github.com/SSS-Vir/media-to-ascii.git
cd media-to-ascii
cargo build --release
```
binary ***hello*** will be in target/release/

## Usage
usage: hello FILE_PATH [OPTIONS]
<br>Options for .gif files:
  <br>&ensp;--fps=FPS     Gif's FPS (default 15)
<br>Options for jpg, jpeg, png:
  <br>&ensp;--nosave      Dont save ascii to a file(default false)
  <br>&ensp;--output FILE Saves ascii to a file(default creates file in working directory)
  <br>&ensp;--size=WIDTHxHEIGHT    Changes size of picture
  <br>&ensp;--noprint     Don't print to console(default false)
<br>General options:
  <br>&ensp;--colored     Makes ascii colored

## Formats:
- [x] .gif
- [x] .jpg
- [x] .jpeg
- [x] .png
- *idk what exists else*

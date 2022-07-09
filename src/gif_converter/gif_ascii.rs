use super::arg_parser::args_parse;
use crate::utils::ascii_utils::ImageBufferExtensions;
use image::{codecs::gif::GifDecoder, AnimationDecoder};
use std::fs::File;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub fn gif_to_ascii(args: &Vec<String>) -> Result<(), &'static str> {
    let (file_path, fps, colored) = match args_parse(args) {
        Ok((path, fps, color)) => (path, fps, color),
        Err(err) => {
            println!("{}", err);
            return Result::Ok(());
        }
    };

    let time_for_frame: u64 = 1000 / fps;

    let file_gif = match File::open(file_path) {
        Ok(x) => x,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    let decoder = match GifDecoder::new(file_gif) {
        Ok(x) => x,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    let frames = match decoder.into_frames().collect_frames() {
        Ok(x) => x,
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };

    clearscreen::clear().expect("failed to clear screen"); // cross-platform screen cleaning

    loop {
        for index in 0..frames.len() {
            let time_start = SystemTime::now();

            let terminal_size = termsize::get().unwrap();
            let new_height = terminal_size.rows;
            let new_width = terminal_size.cols / 2;

            let frame = image::imageops::thumbnail(
                frames[index].buffer(),
                new_width as u32,
                new_height as u32,
            ); //resize frame so it fit in terminal

            if !colored {
                let frame_str = frame.to_ascii_string();
                for line in frame_str {
                    for _ in 0..(new_width / 2) {
                        print!(" ");
                    }
                    print!("{}", line);
                }
            } else {
                let frame_str = frame.to_colored_ascii();
                for line in frame_str {
                    for _ in 0..(new_width / 2) {
                        print!(" ");
                    }
                    for c in line {
                        print!("{}", c);
                    }
                }
            }

            match time_start.elapsed() {
                Ok(time) => {
                    let millis = time.as_millis() as u64;
                    if millis < time_for_frame {
                        sleep(Duration::from_millis(time_for_frame - millis));
                    }
                }
                Err(_err) => {}
            }
        }
    }
}

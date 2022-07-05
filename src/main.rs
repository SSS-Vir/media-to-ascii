mod arg_parser;
mod color_utils;

use std::error::Error;
use image::codecs::gif::GifDecoder;
use std::fs::File;
use std::time::Duration;
use std::time::SystemTime;
use image::{AnimationDecoder};
use std::env::args;
use arg_parser::args_parse;
use color_utils::{determine_brightness, get_colorchar};

fn main() -> Result<(), Box<dyn Error>> {
    let mut file_path: String = String::from("");
    let mut fps: u64 = 24;
    match args_parse(args()) {
        Ok((path, fpss)) => {
            file_path = path;
            fps = fpss;
        }
        Err(err) => {
            println!("{}", err);
            return Ok(());
        }
    };


    let time_for_frame: u64 = 1000 / fps;

    if file_path.is_empty() {
        println!("File was not given or doesn't exist");
        return Ok(());
    }

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

    let mut current_line = String::new();
    let mut all_lines: Vec<String> = Vec::new();

    std::process::Command::new("clear").status().unwrap();

    loop {
        for index in 0..frames.len() {
            let time_start = SystemTime::now();

            let terminal_size = termsize::get().unwrap();
            let new_height = terminal_size.rows;
            let new_width = terminal_size.cols / 2;

            let frame = image::imageops::thumbnail(frames[index].buffer(),
                                                   new_width as u32,
                                                   new_height as u32); //resize frame so it fit in terminal
            let frame = frame.as_raw();
            for j in (0..frame.len()).step_by(4) {
                let r = frame[j] as f32;
                let g = frame[j + 1] as f32;
                let b = frame[j + 2] as f32;
                let brightness = determine_brightness(r, g, b);
                current_line.push(get_colorchar(brightness));
                if j % (new_width * 4) as usize == 0 && j != 0 {
                    current_line.push('\n');
                    all_lines.push(current_line);
                    current_line = String::new();
                }
            }

            for line in &all_lines {
                for _ in 0..(new_width / 2) {
                    print!(" ");
                }
                print!("{}", line);
            };

            current_line.clear();
            all_lines.clear();

            match time_start.elapsed() {
                Ok(time) => {
                    let millis = time.as_millis() as u64;
                    if millis < time_for_frame {
                        std::thread::sleep(Duration::from_millis(time_for_frame - millis));
                    }
                }
                Err(_err) => {}
            }
        }
    }
}
#[path = "../utils/color_utils.rs"]
mod color_utils;
mod arg_parser;

use arg_parser::args_parse;
use color_utils::{determine_brightness, get_colorchar};
use image::codecs::gif::GifDecoder;
use image::AnimationDecoder;
use std::fs::File;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;
use colored::{ColoredString, Colorize};

pub fn gif_to_ascii(args: &Vec<String>) -> Result<(), &'static str> {
    let file_path: String;
    let fps: u64;
    let colored: bool;

    match args_parse(args) {
        Ok((path, fpss, color)) => {
            file_path = path;
            fps = fpss;
            colored = color;
        }
        Err(err) => {
            println!("{}", err);
            return Result::Ok(());
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

    let mut current_line = Vec::<ColoredString>::new();
    let mut all_lines = Vec::<Vec<ColoredString>>::new();

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
            let frame = frame.as_raw();
            for j in (0..frame.len()).step_by(4) {
                let r = frame[j];
                let g = frame[j + 1];
                let b = frame[j + 2];
                let brightness = determine_brightness(&r, &g, &b);
                if colored {
                    current_line.push(get_colorchar(brightness).to_string().truecolor(
                        r,
                        g,
                        b,
                    ));
                }
                else {
                    current_line.push(get_colorchar(brightness).to_string().normal());
                }
                if j % (new_width * 4) as usize == 0 && j != 0 {
                    current_line.push('\n'.to_string().truecolor(255, 255, 255));
                    all_lines.push(current_line);
                    current_line = Vec::<ColoredString>::new();
                }
            }

            for line in &all_lines {
                for _ in 0..(new_width / 2) {
                    print!(" ");
                }
                for c in line {
                    print!("{}", c);
                }
            }

            current_line.clear();
            all_lines.clear();

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

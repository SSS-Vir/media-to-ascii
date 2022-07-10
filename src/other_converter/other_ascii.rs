use crate::utils::ascii_utils::ImageBufferExtensions;
use crate::utils::path_utils::get_file_name;
use std::{fs::File, io::Write};

use super::arg_parser::args_parse;

pub fn picture_to_ascii(args: &Vec<String>) -> Result<(), Box<String>> {
    let (file_path, mut output, size, noprint, colored) = match args_parse(args) {
        Ok((path, out, size, print, color)) => (path, out, size, print, color),
        Err(err) => {
            return Err(err);
        }
    };

    let mut file = match image::open(file_path) {
        Ok(x) => x.to_rgb8(),
        Err(err) => {
            return Err(format!("Error while opening file: {}", err.to_string()).into());
        }
    };

    let mut out_file: File = if output.is_empty() {
        let path_buf = std::env::current_dir().unwrap();
        output = path_buf.to_str().unwrap().to_string();
        output.push_str("\\");
        output.push_str(get_file_name(file_path).as_str());
        output.push_str(".txt");
        match File::create(&output) {
            Ok(x) => x,
            Err(_) => {
                return Err(format!("Can't open file {}", output).into());
            }
        }
    } else {
        match std::fs::File::create(&output) {
            Ok(x) => x,
            Err(_) => {
                return Err(format!("Can't open file {}", output).into());
            }
        }
    };

    if size.0 {
        file = image::imageops::thumbnail(&file, size.1, size.2);
    }

    if colored {
        let new_width = file.width();
        let image_str = file.to_colored_ascii();
        for line in image_str {
            if !noprint {
                for _ in 0..(new_width / 2) {
                    print!(" ");
                }
            }
            for c in line {
                match out_file.write_all(c.as_bytes()) {
                    Ok(_) => {}
                    Err(err) => {
                        return Err(
                            format!("Error while writing in file: {}", err.to_string()).into()
                        );
                    }
                };
                if !noprint {
                    print!("{}", c);
                }
            }
        }
    } else {
        let image_str = file.to_ascii_string();
        let new_width = file.width();
        for line in image_str {
            match out_file.write_all(line.as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    return Err(format!("Error while writing in file: {}", err.to_string()).into());
                }
            };
            if !noprint {
                for _ in 0..(new_width / 2) {
                    print!(" ");
                }
                print!("{}", line);
            }
        }
    }

    Ok(())
}

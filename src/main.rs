mod utils {
    pub mod ascii_utils;
    pub mod color_utils;
    pub mod path_utils;
    #[cfg(windows)]
    pub mod stdout;
}

mod gif_converter {
    pub mod arg_parser;
    pub mod gif_ascii;
}

mod other_converter {
    pub mod arg_parser;
    pub mod other_ascii;
}

mod help;

use gif_converter::gif_ascii::gif_to_ascii;
use std::env::args;
use utils::path_utils::{get_file_extension, is_file, path_exists};

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() < 1 {
        println!("File was not given");
        return Ok(());
    }

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        help::usage_message();
        return Ok(());
    }

    let file_path = &args[0];

    if !path_exists(file_path) {
        println!("{} : path doesn't exist", file_path);
        return Ok(());
    }

    if !is_file(file_path) {
        println!("{} : is not a file", file_path);
        return Ok(());
    }

    let file_extension = get_file_extension(&file_path);

    match file_extension {
        "gif" => {
            match gif_to_ascii(&args) {
                Ok(_) => {}
                Err(err) => {
                    print!("{}", err)
                }
            };
        }

        "" => {
            println!("{} file has no format", file_path);
        }

        ext => {
            let supported = vec!["jpeg", "jpg", "png"];
            if supported.contains(&ext) {
                match other_converter::other_ascii::picture_to_ascii(&args) {
                    Ok(_) => {}
                    Err(err) => {
                        println!("{}", err);
                    }
                }
            } else {
                println!("{} is unsupported format yet", ext);
            }
        }
    }
    Ok(())
}

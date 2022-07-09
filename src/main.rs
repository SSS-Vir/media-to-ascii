#[path = "./gif_converter/gif_ascii.rs"]
mod gif_ascii;

#[path = "./utils/path_utils.rs"]
mod path_utils;

mod help;

use gif_ascii::gif_to_ascii;
use path_utils::{file_exists, get_file_extension, is_file};
use std::env::args;

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

    if !file_exists(file_path) {
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
            gif_to_ascii(&args).expect("idk");
        }

        "" => {
            println!("{} file has no format", file_path);
            return Ok(());
        }

        ext => {
            println!("{} is unsupported format yet", ext);
            return Ok(());
        }
    }

    Ok(())
}

#[path = "./gif_converter/gif_ascii.rs"]
mod gif_ascii;

#[path = "./utils/path_utils.rs"]
mod path_utils;

use path_utils::{file_exists, get_file_extension};
use std::env::args;
use gif_ascii::gif_to_ascii;

fn main() -> Result<(), &'static str> {


    let args: Vec<String> = args().skip(1).collect();
    if args.len() < 1 {
        println!("File was not given");
        return Ok(());
    }

    let file_path = &args[0];

    if !file_exists(file_path) {
        println!("{}: path doesn't exist", file_path);
        return Ok(());
    }

    //let file_extension = "sad";
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

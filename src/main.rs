mod utils {
    pub mod ascii_utils;
    pub mod color_utils;
    pub mod path_utils;
}

mod gif_converter {
    pub mod arg_parser;
    pub mod gif_ascii;
}

mod help;

use gif_converter::gif_ascii::gif_to_ascii;
use std::env::args;
use utils::path_utils::{file_exists, get_file_extension, is_file};

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
            let supported = vec!["jpeg", "jpg", "png"];
            if supported.contains(&ext) {
                todo!();
            } else {
                println!("{} is unsupported format yet", ext);
            }
            return Ok(());
        }
    }

    Ok(())
}

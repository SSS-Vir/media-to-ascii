use std::env::Args;
use std::path::Path;

pub fn args_parse(mut args: Args) -> Result<(String, u64), &'static str> {
    let mut file_path = String::from("");
    let mut fps = 15;
    args.next();
    for arg in args {
        if arg.starts_with("--fps") && fps == 15 {
            fps = parse_fps(&arg);
        } else if file_exists(&arg) && file_path.is_empty() {
            file_path = arg;
        }
    }

    return Result::Ok((file_path, fps));
}

fn parse_fps(arg: &String) -> u64 {
    let default_value = 15;
    let fps_num = arg.split('=').collect::<Vec<&str>>();
    if fps_num.len() != 2 {
        return default_value;
    }
    let fps = match fps_num[1].parse::<u64>() {
        Ok(x) => x,
        Err(_) => default_value,
    };

    return fps;
}

fn file_exists(path: &String) -> bool {
    Path::new(path).exists()
}

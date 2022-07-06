pub fn args_parse(args: &Vec<String>) -> Result<(String, u64), &'static str> {
    let file_path = args[0].clone();
    let mut fps = 15;

    for arg in args {
        if arg.starts_with("--") {
            let attr = arg.chars().skip(2).collect::<String>();

            if attr.starts_with("fps") {
                fps = parse_fps(arg);
            }
        }
    }

    return Ok((file_path, fps));
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


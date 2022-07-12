use crate::utils::path_utils::*;

pub fn args_parse(
    args: &Vec<String>,
) -> Result<(&String, String, (bool, u32, u32), bool, bool, bool), Box<String>> {
    let file_path = &args[0];
    let mut output: String = String::new();
    let mut size: (bool, u32, u32) = (false, 0, 0);
    let mut noprint = false;
    let mut colored = false;
    let mut nosave = false;
    let mut iter = args.iter();
    loop {
        match iter.next() {
            Some(arg) => {
                if arg.starts_with("--") {
                    let attr = arg.chars().skip(2).collect::<String>();

                    if attr.starts_with("output") {
                        if !output.is_empty() {
                            return Err("Output path was given twice".to_string().into());
                        }
                        output = match check_output(iter.next()) {
                            Ok(x) => x,
                            Err(err) => {
                                return Err(err);
                            }
                        };
                    } else if attr.starts_with("size") {
                        if size.0 {
                            return Err("Size was given twice".to_string().into());
                        }

                        size = match parse_size(attr) {
                            Ok(x) => x,
                            Err(err) => {
                                return Err(err);
                            }
                        };
                    } else if attr.starts_with("noprint") {
                        noprint = true;
                    } else if attr.starts_with("colored") {
                        colored = true;
                    } else if attr.starts_with("nosave") {
                        nosave = true;
                    }
                }
            }
            None => break,
        }
    }
    return Ok((file_path, output, size, noprint, colored, nosave));
}

fn check_output(value: Option<&String>) -> Result<String, Box<String>> {
    return match value {
        None => Err("Output file path was not given".to_string().into()),
        Some(path) => {
            if !path_exists(path) {
                let path_os = std::path::Path::new(path);
                match path_os.parent() {
                    Some(parrent_path) => {
                        if !parrent_path.exists() {
                            return Err(format!(
                                "{} : path doesn't exist",
                                parrent_path.to_str().unwrap()
                            )
                            .into());
                        }
                    } // can create file
                    None => return Ok(path.into()),
                }
            } else if !is_file(path) {
                return Err(format!("{}: is not a file", path).into());
            }
            return Ok(path.into());
        }
    };
}

fn parse_size(size: String) -> Result<(bool, u32, u32), Box<String>> {
    let size: Vec<_> = size.split('=').collect();
    if size.len() != 2 {
        return Err("Incorrect size".to_string().into());
    }
    let size: Vec<_> = size[1].split("x").collect();

    if size.len() != 2 {
        return Err("Incorrect size".to_string().into());
    }

    let width = match size[0].parse::<u32>() {
        Ok(x) => x,

        Err(err) => {
            return Err(format!("Error occurred while parsing width: {}", err).into());
        }
    };

    let height = match size[1].parse::<u32>() {
        Ok(x) => x,

        Err(err) => {
            return Err(format!("Error occurred while parsing height: {}", err).into());
        }
    };

    return Ok((true, width, height));
}

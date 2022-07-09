use std::path::Path;

pub fn path_exists(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn get_file_extension(path: &String) -> &str {
    return match Path::new(path).extension() {
        None => "",
        Some(ext) => ext.to_str().unwrap(),
    };
}

pub fn is_file(path: &String) -> bool {
    Path::new(path).is_file()
}

pub fn get_file_name(path: &String) -> String {
    let p = Path::new(path);
    p.file_name().unwrap().to_str().unwrap().into()
}

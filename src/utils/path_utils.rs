use std::path::Path;

pub fn file_exists(path: &String) -> bool {
    Path::new(path).exists()
}

pub fn get_file_extension(path: &String) -> &str {
    return match Path::new(path).extension() {
        None => { "" }
        Some(ext) => { ext.to_str().unwrap() }
    }
}
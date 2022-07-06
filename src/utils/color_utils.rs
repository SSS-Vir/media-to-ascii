pub fn get_colorchar(brightness: f32) -> char {
    let gradient = String::from(" .:!/r(l1Z4H9W8$@"); // onigiri ))))()
    let step = 255.0 / gradient.len() as f32;
    for i in 0..gradient.len() {
        if brightness < step * (i + 1) as f32 {
            return gradient.chars().nth(i).unwrap();
        }
    }
    return 'U';
}

pub fn determine_brightness(r: &u8, g: &u8, b: &u8) -> f32 {
    return 0.2126 * (r.clone() as f32) + 0.7152 * (g.clone() as f32) + 0.0722 * (b.clone() as f32); // magic
}
use crate::utils::color_utils::*;
use colored::{ColoredString, Colorize};
use image::{ImageBuffer, Pixel};

pub trait ImageBufferExtensions {
    fn to_ascii_string(&self) -> Vec<String>;
    fn to_colored_ascii(&self) -> Vec<Vec<ColoredString>>;
}

impl<P: Pixel> ImageBufferExtensions for ImageBuffer<P, Vec<P::Subpixel>> {
    fn to_ascii_string(&self) -> Vec<String> {
        let mut str_image = Vec::<String>::new();
        let width = self.width() as usize;
        #[cfg(windows)]
        let default_line_vals: String = String::from_utf8(vec![' ' as u8; width / 2]).unwrap();
        #[cfg(not(windows))]
        let default_line_vals: String = String::new();
        let mut line = default_line_vals.clone();
        let mut pixels = self.pixels();
        for i in 0..pixels.len() {
            let pixel = pixels.next().unwrap();
            let c = get_colorchar(pixel.brightness());
            line.push(c);
            if (i + 1) % width == 0 {
                line.push('\n');
                str_image.push(line);
                line = default_line_vals.clone();
            }
        }

        str_image
    }

    fn to_colored_ascii(&self) -> Vec<Vec<ColoredString>> {
        let mut str_image = Vec::<Vec<ColoredString>>::new();
        let mut line = Vec::<ColoredString>::new();
        let width = self.width() as usize;
        let mut pixels = self.pixels();
        for i in 0..pixels.len() {
            let pixel = pixels.next().unwrap();
            let c = get_colorchar(pixel.brightness());
            let color = get_pixel_truecolor(pixel);
            line.push(c.to_string().truecolor(color.0, color.1, color.2));
            if (i + 1) % width == 0 {
                line.push("\n".normal());
                str_image.push(line);
                line = Vec::<ColoredString>::new();
            }
        }

        str_image
    }
}

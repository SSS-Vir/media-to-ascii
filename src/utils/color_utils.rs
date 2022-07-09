use image::{Pixel, Primitive};

trait Cast: Primitive {
    fn to_f64(&self) -> f64;
    fn to_u8(&self) -> u8;
}

// super mega costil
impl<P> Cast for P
where
    P: Primitive,
{
    fn to_f64(&self) -> f64 {
        self.to_f64().unwrap()
    }

    fn to_u8(&self) -> u8 {
        self.to_u8().unwrap()
    }
}

pub trait PixelExtensions {
    fn brightness(&self) -> f64;
}

impl<P> PixelExtensions for P
where
    P: Pixel,
{
    fn brightness(&self) -> f64 {
        self.to_luma().0[0].to_f64()
    }
}

pub fn get_pixel_truecolor(pixel: &impl Pixel) -> (u8, u8, u8) {
    let rgb = pixel.to_rgb();
    (rgb.0[0].to_u8(), rgb.0[1].to_u8(), rgb.0[2].to_u8())
}

pub fn get_colorchar(brightness: f64) -> char {
    let gradient = String::from(" .:!/r(l1Z4H9W8$@"); // onigiri ))))()
    let step = 255.0 / gradient.len() as f64;
    for i in 0..gradient.len() {
        if brightness < step * (i + 1) as f64 {
            return gradient.chars().nth(i).unwrap();
        }
    }
    return 'U';
}

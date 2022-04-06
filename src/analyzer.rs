
use image::{GenericImageView, DynamicImage, ImageError};


pub struct Analyzer {
    pub data: DynamicImage
}

impl Analyzer {

    pub fn from(path: &str) -> Result<Self, ImageError> {
        Ok(Self {
            data: image::open(path)?
        })
    }

    pub fn start(&self) {
        for pixel in self.data.pixels() {
            println!("Pixel x: {}, y: {}, color: {:?}", pixel.0, pixel.1, pixel.2);
        }
    }

}
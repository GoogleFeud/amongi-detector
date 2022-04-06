
use image::{GenericImageView, DynamicImage, ImageError, Rgba};
use crate::detector::Detector;

pub type Pixel = (u32, u32, Rgba<u8>);

pub trait MovePixel {
    fn right(&self, data: &DynamicImage) -> Option<Rgba<u8>>;
    fn top(&self, data: &DynamicImage) -> Option<Rgba<u8>>;
    fn bottom(&self, data: &DynamicImage) -> Option<Rgba<u8>>;
    fn left(&self, data: &DynamicImage) -> Option<Rgba<u8>>;
}

impl MovePixel for Pixel {

    fn right(&self, data: &DynamicImage) -> Option<Rgba<u8>> {
        if self.0 < data.width() {
            Some(data.get_pixel(self.0 + 1, self.1))
        } else {
            None
        }
    }

    fn left(&self, data: &DynamicImage) -> Option<Rgba<u8>> {
        if self.0 > 0 {
            Some(data.get_pixel(self.0 - 1, self.1))
        } else {
            None
        }
    }

    fn top(&self, data: &DynamicImage) -> Option<Rgba<u8>> {
        if self.1 > 0 {
            Some(data.get_pixel(self.0, self.1 - 1))
        } else {
            None
        }
    }

    fn bottom(&self, data: &DynamicImage) -> Option<Rgba<u8>> {
        if self.1 > data.width() {
            Some(data.get_pixel(self.0, self.1 + 1))
        } else {
            None
        }
    }
    
}


pub struct Analyzer {
    pub data: DynamicImage
}

impl Analyzer {

    pub fn from(path: &str) -> Result<Self, ImageError> {
        Ok(Self {
            data: image::open(path)?
        })
    }

    pub fn run<T: Detector>(&self, mut detectors: Vec<&mut T>) {
        for pixel in self.data.pixels() {
            for detector in detectors.iter_mut() {
                detector.on_pixel(self, pixel);
            }
        }
    }

}

use image::{GenericImageView, DynamicImage, ImageError, Rgba};
use crate::detector::Detector;

pub type Pixel = (u32, u32, Rgba<u8>);

pub trait MovePixel {
    fn right(&self, data: &DynamicImage, inc: u32) -> Option<Pixel>;
    fn up(&self, data: &DynamicImage, inc: u32) -> Option<Pixel>;
    fn down(&self, data: &DynamicImage, inc: u32) -> Option<Pixel>;
    fn left(&self, data: &DynamicImage, inc: u32) -> Option<Pixel>;
}

impl MovePixel for Pixel {

    fn right(&self, data: &DynamicImage, inc: u32) -> Option<Pixel> {
        let new_total = self.0 + inc;
        if new_total < data.width() {
            Some((new_total, self.1, data.get_pixel(new_total, self.1)))
        } else {
            None
        }
    }

    fn left(&self, data: &DynamicImage, inc: u32) -> Option<Pixel> {
        if self.0 > inc {
            Some((self.0 - inc, self.1, data.get_pixel(self.0 - inc, self.1)))
        } else {
            None
        }
    }

    fn up(&self, data: &DynamicImage, inc: u32) -> Option<Pixel> {
        if self.1 > inc {
            Some((self.0, self.1 - inc, data.get_pixel(self.0, self.1 - inc)))
        } else {
            None
        }
    }

    fn down(&self, data: &DynamicImage, inc: u32) -> Option<Pixel> {
        let new_total = self.1 + inc;
        if new_total < data.width() {
            Some((self.0, new_total, data.get_pixel(self.0, new_total)))
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

    pub fn run<T: Detector>(&self, mut detectors: Vec<&mut T>) -> Vec<Pixel> {
        let mut marked_pixels: Vec<Pixel> = vec![];
        for pixel in self.data.pixels() {
            for detector in detectors.iter_mut() {
                if let Some(mut pixels) = detector.on_pixel(self, &pixel) {
                    marked_pixels.append(&mut pixels);
                }
            }
        };
        marked_pixels
    }

}
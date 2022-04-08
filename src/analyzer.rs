use image::{GenericImageView, DynamicImage, ImageError, Rgba, GenericImage};
use crate::detector::Detector;
use rayon::prelude::*;

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

    pub fn run(&self, detectors: Vec<&dyn Detector>) -> Vec<Pixel> {
        self.data.pixels().par_bridge().fold_with(vec![], |mut a: Vec<Pixel>, pixel| {
            for detector in detectors.iter() {
                // If the detector returns `Some`, then we add the returned pixels to
                // the "marked_pixels" vector
                if let Some(mut pixels) = detector.on_pixel(self, &pixel) {
                    a.append(&mut pixels);
                }
            }
            a
        }).flatten().collect()
    }

    pub fn highlight(&self, pixels: Vec<Pixel>) -> DynamicImage {
        let mut new_image = self.data.brighten(-200);
        for pixel in pixels {
            new_image.put_pixel(pixel.0, pixel.1, pixel.2);
        };
        new_image
    }

}
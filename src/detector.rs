
use crate::analyzer::{Analyzer, Pixel, MovePixel};

pub trait Detector {
    fn on_pixel(&mut self, analyer: &Analyzer, pixel: Pixel);
}

pub struct AmongiDetector {}

impl Detector for AmongiDetector {

    fn on_pixel(&mut self, analyer: &Analyzer, pixel: Pixel) {
        println!("Pixel {:?}, on top: {:?}", pixel, pixel.top(&analyer.data))
    }
}

impl AmongiDetector {
    
    pub fn new() -> Self {
        Self {}
    }

    fn collect(&mut self) -> u32 {
        return 1;
    }
}
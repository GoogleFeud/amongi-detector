
use crate::analyzer::{Analyzer, Pixel, MovePixel};
use image::{Rgba};
use std::collections::HashMap;

pub trait Detector {
    fn on_pixel(&mut self, analyzer: &Analyzer, pixel: &Pixel) -> Option<Vec<Pixel>>;
}

macro_rules! cmp_pixel {
    ($analyzer: expr, $pixel: expr, $res: expr, ($(($($direction: ident $amount: expr)=>+)),+)) => {
        $(
            let upper = $pixel.$($direction(&$analyzer.data, $amount)?).+;
            if $pixel.2 == upper.2 {
                $res.push(upper);
            } else {
                return None;
            }
        )+
    };
}

pub struct AmongiDetector {
    pub results: HashMap<Rgba<u8>, u32>
}

impl Detector for AmongiDetector {

    fn on_pixel(&mut self, analyzer: &Analyzer, pixel: &Pixel) -> Option<Vec<Pixel>> {
        let mut res: Vec<Pixel> = vec![];
        // This compares the color of "pixel" with the provided relative coordinates. This is the outer layer of the 
        // among us character. We compare it's eyes later on.
        cmp_pixel!(analyzer, pixel, res, (
            // It's head?
            (right 1),
            (right 2),
            // It's back
            (down 1),
            (down 2),
            (down 3),
            // It's legs
            (down 4),
            (down 4 => right 2),
            // It's torso
            (down 2 => right 1),
            (down 2 => right 2),
            (down 3 => right 1),
            (down 3 => right 2),
            // Backpack / oxygen tank whatever
            (left 1 => down 1),
            (left 1 => down 2)
        ));
        let eye_1 = pixel.down(&analyzer.data, 1)?.right(&analyzer.data, 1)?;
        // If the eye is the same color as it's body, then it's not really amongus
        if eye_1.2 == pixel.2 {
            return None;
        }
        let eye_2 = eye_1.right(&analyzer.data, 1)?;
        if eye_1.2 == eye_2.2 {
            res.push(eye_1);
            res.push(eye_2);
            if let Some(amount) = self.results.remove(&pixel.2) {
                self.results.insert(pixel.2, amount + 1);
            } else {
                self.results.insert(pixel.2, 1);
            }
            Some(res)
        } else {
            None
        }
    }
}

impl AmongiDetector {
    
    pub fn new() -> Self {
        Self {
            results: HashMap::new()
        }
    }

}
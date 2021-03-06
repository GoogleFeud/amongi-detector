use crate::{analyzer::{Analyzer, Pixel, MovePixel}, detector::cmp_close_pixel};
use image::{Rgba};
use std::collections::HashMap;
use crate::detector::{Detector, cmp_pixel};
pub struct AmongiDetector {
    pub results: HashMap<Rgba<u8>, u32>
}

impl Detector for AmongiDetector {

    fn on_pixel(&mut self, analyzer: &Analyzer, pixel: &Pixel) -> Option<Vec<Pixel>> {
        let mut res: Vec<Pixel> = vec![];
        let eye_1 = pixel.down(&analyzer.data, 1)?.right(&analyzer.data, 1)?;
        let eye_2 = eye_1.right(&analyzer.data, 1)?;
        if eye_1.2 == pixel.2 || eye_1.2 != eye_2.2 {
            return None
        }
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
        cmp_close_pixel!(analyzer, pixel, res, 6, ==, (
            // Pixels above head
            (up 1, 1),
            (right 1 => up 1, 1),
            (right 2 => up 2, 1),
            // Pixels around head
            (right 3, 1),
            (left 1, 1),
            // Pixels right of the oxygen tank
            (down 1 => left 2, 1),
            (down 2 => left 2, 1),
            (down 3 => left 1, 1),
            // Checking pixels below both legs
            (down 5, 1),
            (down 5 => right 2, 1),
            // Pixels in front
            (down 2 => left 3, 1),
            (down 3 => left 3, 1),
            (down 4 => left 3, 1)
        ));
        res.push(eye_1);
        res.push(eye_2);
        res.push(*pixel);
        if let Some(amount) = self.results.remove(&pixel.2) {
            self.results.insert(pixel.2, amount + 1);
        } else {
            self.results.insert(pixel.2, 1);
        }
        Some(res)
    }
}

impl AmongiDetector {
    
    pub fn new() -> Self {
        Self {
            results: HashMap::new()
        }
    }

}

impl Default for AmongiDetector {
    fn default() -> Self {
        Self::new()
    }
}
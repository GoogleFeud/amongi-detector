use crate::analyzer::{Analyzer, Pixel, MovePixel};
use image::{Rgba};
use std::collections::HashMap;
use crate::detector::{Detector, cmp_pixel};
use std::sync::Mutex;

pub struct AmongiDetector {
    pub results: Mutex<HashMap<Rgba<u8>, u32>>
}

impl Detector for AmongiDetector {

    fn on_pixel(&self, analyzer: &Analyzer, pixel: &Pixel) -> Option<Vec<Pixel>> {
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
        res.push(eye_1);
        res.push(eye_2);
        res.push(*pixel);
        let mut map = self.results.lock().unwrap();
        if let Some(amount) = map.remove(&pixel.2) {
            map.insert(pixel.2, amount + 1);
        } else {
            map.insert(pixel.2, 1);
        }
        Some(res)
    }
}

impl AmongiDetector {
    
    pub fn new() -> Self {
        Self {
            results: Mutex::new(HashMap::new())
        }
    }

}

impl Default for AmongiDetector {
    fn default() -> Self {
        Self::new()
    }
}
mod detector;
mod analyzer;
mod detectors;
pub use analyzer::{Analyzer};
pub use detectors::amongi::AmongiDetector;
use std::time::{Instant};


fn main() {
    let analyzer = Analyzer::from("./place.png").expect("Image doesn't exist.");
    let amongi_collector = AmongiDetector::new();
    let before = Instant::now();
    let pixels = analyzer.run(vec![&amongi_collector]);
    analyzer.highlight(pixels).save_with_format("./result.png", image::ImageFormat::Png).expect("Couldn't save image");
    let total_amongi = amongi_collector.results.lock().unwrap().values().fold(0, |a, b| a + b);
    println!("Found {} amongi", total_amongi);
    println!("Took {} seconds", before.elapsed().as_secs_f32());
}

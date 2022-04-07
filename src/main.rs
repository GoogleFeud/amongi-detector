mod detector;
mod analyzer;
mod detectors;
pub use analyzer::{Analyzer};
pub use detectors::amongi::AmongiDetector;

fn main() {
    let analyzer = Analyzer::from("./place.png").expect("Image doesn't exist.");
    let mut amongi_collector = AmongiDetector::new();
    let pixels = analyzer.run(vec![&mut amongi_collector]);
    analyzer.highlight(pixels).save_with_format("./result.png", image::ImageFormat::Png).expect("Couldn't save image");
}

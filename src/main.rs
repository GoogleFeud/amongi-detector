mod detector;
mod analyzer;
pub use analyzer::{Analyzer};
use detector::AmongiDetector;

fn main() {
    let analyzer = Analyzer::from("./image.png").expect("Image doesn't exist.");
    let mut amongi_collector = AmongiDetector::new();
    println!("{:?}", analyzer.run(vec![&mut amongi_collector]));
}

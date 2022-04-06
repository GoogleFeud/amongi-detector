mod detector;
mod analyzer;
mod detectors;
pub use analyzer::{Analyzer};
pub use detectors::amongi::AmongiDetector;

fn main() {
    let analyzer = Analyzer::from("./place.png").expect("Image doesn't exist.");
    let mut amongi_collector = AmongiDetector::new();
    analyzer.run(vec![&mut amongi_collector]);
    println!("{:?}", amongi_collector.results);
}

mod analyzer;

pub use analyzer::{Analyzer};

fn main() {
    let analyzer = Analyzer::from("./image.png").expect("Image doesn't exist.");
    analyzer.start();
}

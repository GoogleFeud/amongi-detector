mod detector;
mod analyzer;
mod detectors;
pub use analyzer::{Analyzer};
pub use detectors::amongi::AmongiDetector;
use std::time::{Instant};
use argh::FromArgs;

#[derive(FromArgs)]
/// Detect among us characters
struct Args {
    #[argh(option)]
    /// the image
    pub image: String,
    #[argh(option)]
    /// shows time
    pub time: Option<bool>,
    #[argh(option)]
    /// where to store the final result
    pub out: Option<String>
}

fn main() {
    let args = argh::from_env::<Args>();
    let analyzer = Analyzer::from(&args.image).expect("Image doesn't exist.");
    let mut amongi_collector = AmongiDetector::new();
    let before = Instant::now();
    let pixels = analyzer.run(vec![&mut amongi_collector]);
    analyzer.highlight(pixels).save_with_format(args.out.unwrap_or("./result.png".to_string()), image::ImageFormat::Png).expect("Couldn't save image.");
    let total_amongi = amongi_collector.results.values().sum::<u32>();
    println!("Found {} amongi", total_amongi);
    if let Some(show_time) = args.time {
        if show_time {
            println!("Took {} seconds", before.elapsed().as_secs_f32());
        }
    }
}

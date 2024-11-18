use opencv::knn_background_subtraction_opencv;
use ::opencv::{imgcodecs, core};
use std::time::Instant;
use util::find_likely_balls;

mod opencv;
mod util;
fn opencv() {
    let start = Instant::now();
    let video_path = "TestVideo.mp4";
    knn_background_subtraction_opencv(video_path).unwrap();
    let duration = start.elapsed();
    println!("Function call took: {:?}", duration);
}

fn draw_contours() {
    let frame = imgcodecs::imread("IMG1.png", imgcodecs::IMREAD_COLOR).unwrap();
    let a = find_likely_balls(&frame);
    println!("About to save image");
    let _ = imgcodecs::imwrite("contours.png", &a, &core::Vector::new());
    println!("Saved image");
}

fn main() {
    draw_contours();
    opencv();
}

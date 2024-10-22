use opencv::knn_background_subtraction_opencv;
use std::time::Instant;

mod opencv;
fn opencv() {
    let start = Instant::now();
    let video_path = "TestVideo.mp4";
    knn_background_subtraction_opencv(video_path).unwrap();
    let duration = start.elapsed();
    println!("Function call took: {:?}", duration);
}
fn main() {
    opencv();
}

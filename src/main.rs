use opencv::knn_background_subtraction_opencv;
use ::opencv::imgcodecs;
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
        match find_likely_balls(&frame) {
            Ok(contour_image) => {
                // Do something with the contour image
                println!("Contours drawn and saved to contours.png");
            },
            Err(e) => println!("Error: {:?}", e),
        };
}

fn main() {
    draw_contours();
    opencv();
}


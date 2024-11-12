use opencv::knn_background_subtraction_opencv;
use ::opencv::{imgcodecs, core};
use std::time::Instant;
use util::find_likely_balls;

mod opencv;
mod util;
fn opencv() {
    // This calls and records how long the knn_background subtraction takes
    let start = Instant::now();
    let video_path = "TestVideo.mp4";
    knn_background_subtraction_opencv(video_path).unwrap();
    let duration = start.elapsed();
    println!("Function call took: {:?}", duration);
}

fn draw_contours() {
    let frame = imgcodecs::imread("IMG3.png", imgcodecs::IMREAD_COLOR).unwrap();
        match find_likely_balls(&frame) {
            Ok(contour_image) => {
                let _ = imgcodecs::imwrite("contours.png", &contour_image, &core::Vector::new());
            },
            Err(e) => println!("Error: {:?}", e),
        };
}

fn main() {
    draw_contours();
    opencv();
}



// rebase comment, happened second

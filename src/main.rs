use image::open;
use opencv::knn_background_subtraction_opencv;
use std::time::Instant;

mod knn;
mod opencv;

fn two_images() {
    // Load IMG1 and IMG2 into buffer
    let img1 = open("IMG1.png").unwrap();
    let img2 = open("IMG2.png").unwrap();

    // Start the timer
    let start = Instant::now();

    // Call the function
    let img3 = knn::knn_background_subtraction(img1, img2, 110);

    // Measure the duration
    let duration = start.elapsed();
    println!("Function call took: {:?}", duration);

    // Save the result to a file
    img3.save("IMG3.png").unwrap();

}

fn multiple_images() {
    // Load IMG1 and IMG2 into buffer
    let img1 = open("IMG1.png").unwrap();
    let img2 = open("IMG2.png").unwrap();

    // Make a vector of images
    let images = vec![img1, img2];

    // Start the timer
    let start = Instant::now();

    // Call the function
    let img3 = knn::knn(images, 150);

    // Measure the duration
    let duration = start.elapsed();
    println!("Function call took: {:?}", duration);

    // Save the result to a file
    img3.save("IMG5.png").unwrap();
}

fn opencv() {
    let start = Instant::now();
    let video_path = "TestVideo.mp4";
    knn_background_subtraction_opencv(video_path).unwrap();
    let duration = start.elapsed();
    println!("Function call took: {:?}", duration);
}
fn main() {
    two_images();
    multiple_images();
    opencv();
}

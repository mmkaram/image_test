use std::time::Instant;
use image::open;

mod knn;

fn main() {
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
    img3.save("IMG4.png").unwrap();
}

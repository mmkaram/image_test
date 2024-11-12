use opencv::{
    core::{Point, Scalar, Vector},
    imgproc,
    prelude::*,
    imgcodecs,
};

pub fn find_likely_balls(frame: &Mat) -> Result<Mat, opencv::Error> {

    // Convert the image to grayscale
    let mut gray = Mat::default();
    imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // Apply thresholding to create a binary image
    let mut binary = Mat::default();
    imgproc::threshold(&gray, &mut binary, 127.0, 255.0, imgproc::THRESH_BINARY)?;

    let mut contours = Vector::<Vector<Point>>::new();
    
    imgproc::find_contours(
        frame,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0)
    )?;

    // Create a copy of the original frame to draw contours on
    let mut contour_image = frame.clone();
    // Draw all contours
    imgproc::draw_contours(
        &mut contour_image,
        &contours,
        -1, // -1 means draw all contours
        Scalar::new(0.0, 255.0, 0.0, 255.0), // Green color
        2, // Thickness
        imgproc::LINE_8,
        &Mat::default(), // No hierarchy
        0, // Max level
        Point::new(0, 0)
    )?;

    // Optionally, save the image
    imgcodecs::imwrite("contours.png", &contour_image, &Vector::new())?;

    Ok(contour_image)
}

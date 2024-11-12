use opencv::{
    core::{Mat, Point, Scalar, Vector},
    imgproc,
    prelude::*,
};

pub fn find_likely_balls(frame: &Mat) -> Result<Mat, opencv::Error> {
    // Create a working copy of the frame
    let mut working_mat = frame.clone();

    // For testing: Create a simple image with a circle
    // In real usage, you'd remove this and use the actual frame
    let _ = working_mat.set_to(&Scalar::all(0.0), &Mat::default());
    imgproc::circle(
        &mut working_mat,
        Point::new(100, 100),
        30,
        Scalar::new(255.0, 255.0, 255.0, 0.0),
        -1,
        imgproc::LINE_8,
        0,
    )?;

    // Convert to grayscale if not already
    let mut gray_mat = Mat::default();
    if frame.channels() > 1 {
        imgproc::cvt_color(&working_mat, &mut gray_mat, imgproc::COLOR_BGR2GRAY, 0)?;
    } else {
        gray_mat = working_mat.clone();
    }

    // Apply threshold to create binary image
    let mut binary = Mat::default();
    imgproc::threshold(
        &gray_mat,
        &mut binary,
        127.0,
        255.0,
        imgproc::THRESH_BINARY,
    )?;

    // Find contours
    let mut contours: Vector<Vector<Point>> = Vector::new();
    let hierarchy = Mat::default();
    
    imgproc::find_contours(
        &binary,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )?;

    // Create output image for visualization
    let mut output = frame.clone();
    
    // Draw all contours
    for (i, contour) in contours.iter().enumerate() {
        imgproc::draw_contours(
            &mut output,
            &contours,
            i as i32,
            Scalar::new(0.0, 255.0, 0.0, 0.0), // Green color
            2,                                  // Thickness
            imgproc::LINE_8,
            &hierarchy,
            0,
            Point::new(0, 0),
        )?;
    }

    Ok(output)
}

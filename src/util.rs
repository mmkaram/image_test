use opencv::{
    core::{Mat, Point, Vector, CV_8U},
    imgproc,
};

fn convert_to_grayscale(input_image: &Mat) -> Result<Mat, opencv::Error> {
    let mut gray_image = Mat::default();
    imgproc::cvt_color(input_image, &mut gray_image, imgproc::COLOR_BGRA2GRAY, 0)?;
    Ok(gray_image)
}

use opencv::prelude::MatTraitConst;
fn convert_to_8bit(input_image: &Mat) -> Result<Mat, opencv::Error> {
    let mut dst = Mat::default();
    
    // Get the depth
    let depth = MatTraitConst::depth(input_image);
    
    if depth != CV_8U {
        // Convert to 8-bit
        input_image.convert_to(&mut dst, CV_8U, 1.0, 0.0)?;
    } else {
        // If it's already 8-bit, just clone the source
        dst = input_image.clone();
    }
    
    Ok(dst)
}

fn make_binary(unclean_image: &Mat) -> Result<Mat, opencv::Error> {
    let grayscale = convert_to_grayscale(&unclean_image);
    let eight_bit = convert_to_8bit(&grayscale?);
    return eight_bit;
}


pub fn find_likely_balls(image: &Mat) ->  Vector<Vector<Point>> {
    let mut contours: Vector<Vector<Point>> = Vector::new();
    let binary_image = make_binary(&image);
    imgproc::find_contours(
        &binary_image.unwrap(),
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0, 0),
    )
    .unwrap();

    // TODO: overlay contours on image

    return contours;


}

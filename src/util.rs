use opencv::{core::{Point, Vector}, imgproc};

pub fn find_likely_balls(frame: &opencv::prelude::Mat) {

    let mut contours= Vector::<Point>::new();

    let _ = imgproc::find_contours(
        frame,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::new(0,0)
    );

}

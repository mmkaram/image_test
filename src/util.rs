pub fn find_likely_balls(frame: &opencv::prelude::Mat) {

    let mut contours = opencv::types::VectorOfPoint::new();

    let _ = opencv::imgproc::find_contours(
        frame,
        &mut contours,
        opencv::imgproc::RETR_EXTERNAL,
        opencv::imgproc::CHAIN_APPROX_SIMPLE,
        opencv::core::Point::new(0,0)
    );

}

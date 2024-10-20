use opencv::{
    highgui, prelude::*, video, videoio, Error
};

const VIDEO_DEBUG: bool = false;

pub fn knn_background_subtraction_opencv(video_path: &str) -> Result<(), Error> {
    // Open the video file
    let mut cap = videoio::VideoCapture::from_file(video_path, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    let mut fg_mask = Mat::default();

    // Create a KNN Background Subtractor
    let mut bg_subtractor = video::create_background_subtractor_knn(500, 100.0, false)?;

    if VIDEO_DEBUG {
        // Open a GUI window
        highgui::named_window("Foreground Mask", highgui::WINDOW_NORMAL)?;
    }

    // Process the video frame by frame
    loop {
        // Read a frame from the video
        if !cap.read(&mut frame)? || frame.size()?.width == 0 {
            break; // Exit loop if no more frames
        }

        // Apply the KNN background subtractor
        bg_subtractor.apply(&frame, &mut fg_mask, -1.0)?;

        if VIDEO_DEBUG {
            // Show the foreground mask
            highgui::imshow("Foreground Mask", &fg_mask)?;
        
            // Exit on key press
            let key = highgui::wait_key(30)?;
            if key == 113 { // quit with 'q'
                break;
            }
        }
    }

    if VIDEO_DEBUG {
    // Release resources
        highgui::destroy_all_windows()?;
    }
    Ok(())
}

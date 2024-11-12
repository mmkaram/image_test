use opencv::{
    core::Size, highgui, imgproc::{cvt_color, COLOR_GRAY2BGR}, prelude::*, video, videoio::{VideoCapture, self}, Error
};
use super::util::find_likely_balls;

const VIDEO_DEBUG: bool = false;
const VIDEO_OUTPUT_DEBUG: bool = false;

pub fn knn_background_subtraction_opencv(video_path: &str) -> Result<(), Error> {
    // Open the video file
    let mut cap = VideoCapture::from_file(video_path, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    let mut fg_mask = Mat::default();

    // Create a KNN Background Subtractor
    let mut bg_subtractor = video::create_background_subtractor_knn(500, 100.0, false)?;

    if VIDEO_DEBUG {
        // Open a GUI window
        highgui::named_window("Foreground Mask", highgui::WINDOW_NORMAL)?;
    }

    let mut buffer: Option<videoio::VideoWriter> = None;

    if VIDEO_OUTPUT_DEBUG {
        let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
        let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

        // Create a Size instance
        let frame_size = Size::new(width, height);

        // Open a video buffer to add frames to
        buffer = Some(
            videoio::VideoWriter::new(
                "target/output.mp4",
                videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?,
                cap.get(videoio::CAP_PROP_FPS)?,
                frame_size,
                true,
            )?
        );
    }

    // Process the video frame by frame
    loop {
        // Read a frame from the video
        if !cap.read(&mut frame)? || frame.size()?.width == 0 {
            break; // Exit loop if no more frames
        }

        // Apply the KNN background subtractor
        bg_subtractor.apply(&frame, &mut fg_mask, -1.0)?;

        // this is where we should be looking for the ball
        find_likely_balls(&frame);

        if VIDEO_DEBUG {
            // Show the foreground mask
            highgui::imshow("Foreground Mask", &fg_mask)?;
        
            // Exit on key press
            let key = highgui::wait_key(30)?;
            if key == 113 { // quit with 'q'
                break;
            }
        }
        if VIDEO_OUTPUT_DEBUG {
            let mut color_frame = Mat::default();
    
            // Convert fg_mask to a 3-channel BGR image for output
            cvt_color(&fg_mask, &mut color_frame, COLOR_GRAY2BGR, 0)?;
            
            // Write the converted frame to the video buffer
            buffer.as_mut().unwrap().write(&color_frame)?;
        }
    }

    if VIDEO_DEBUG {
    // Release resources
        highgui::destroy_all_windows()?;
    }

    if VIDEO_OUTPUT_DEBUG {
        // Release resources
        buffer.unwrap().release()?;
    }
    Ok(())
}

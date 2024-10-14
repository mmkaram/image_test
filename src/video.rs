use gstreamer::prelude::*;
use gstreamer_video::prelude::*;
use image::{DynamicImage, RgbaImage};
use std::error::Error;

fn extract_frames(video_path: &str, num_frames: usize) -> Result<Vec<DynamicImage>, Box<dyn Error>> {
    gstreamer::init()?;

    let pipeline = format!("filesrc location={} ! decodebin ! videoconvert ! appsink", video_path);
    let pipeline = gstreamer::parse_launch(&pipeline)?;

    let sink = pipeline
        .dynamic_cast::<gstreamer::Element>()
        .unwrap()
        .get_by_name("appsink")
        .unwrap();

    pipeline.set_state(gstreamer::State::Playing)?;

    let mut frames = Vec::new();

    let mut sample_count = 0;

    while sample_count < num_frames {
        let sample = sink.get_sample()?;
        if let Some(buffer) = sample.buffer() {
            let caps = sample.caps().unwrap();
            let info = gstreamer_video::VideoInfo::from_caps(&caps).unwrap();

            // Convert GStreamer buffer to an image
            let width = info.width();
            let height = info.height();
            let size = (width * height * 4) as usize; // Assuming RGBA
            let mut data = vec![0u8; size];

            buffer.map_readable(|b| {
                data.copy_from_slice(b.as_ref());
                Ok(())
            })?;

            let img: RgbaImage = ImageBuffer::from_raw(width, height, data).ok_or("Failed to create image buffer")?;
            frames.push(DynamicImage::ImageRgba8(img));

            sample_count += 1;
        }
    }

    pipeline.set_state(gstreamer::State::Null)?;
    Ok(frames)
}

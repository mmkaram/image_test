use image::{Rgb, RgbImage, DynamicImage, GenericImageView};

pub fn knn_background_subtraction(img1: DynamicImage, img2: DynamicImage, thresh: u8) -> DynamicImage {
    // both images are the same size
    assert_eq!(img1.dimensions(), img2.dimensions());

    // get the width of the images
    let width = img1.width();
    // get the height of the images
    let height = img1.height();

    // preapre new image with same size as the images
    let mut img3 = RgbImage::new(width, height);

    // loop through the width
    for x in 0..width {
        // loop through the height
        for y in 0..height {
            // get the pixel at the current position
            let pixel1 = img1.get_pixel(x, y);
            let pixel2 = img2.get_pixel(x, y);

            // calculate the absolute differences for each color channel
            let diff_red = (pixel1[0] as i16 - pixel2[0] as i16).abs() as u8;
            let diff_green = (pixel1[1] as i16 - pixel2[1] as i16).abs() as u8;
            let diff_blue = (pixel1[2] as i16 - pixel2[2] as i16).abs() as u8;

            // check if any difference exceeds the threshold
            if diff_red > thresh || diff_green > thresh || diff_blue > thresh {
                img3.put_pixel(x, y, Rgb([255, 255, 255])); // pixel changed, make it black
            } else {
                img3.put_pixel(x, y, Rgb([0, 0, 0])); // pixel unchanged, make it white
            }
        }
    }
    return DynamicImage::ImageRgb8(img3);
}

pub fn average_images(images: Vec<DynamicImage>, thresh: u8) -> DynamicImage {
    // Ensure there is at least one image
    assert!(!images.is_empty());

    let num_images = images.len() as u32;

    // Get the dimensions from the first image
    let (width, height) = images[0].dimensions();
    
    // Create an image to store the average
    let mut avg_image = RgbImage::new(width, height);
    
    // Initialize a vector to store the sum of pixel values
    let mut sum = vec![0u32; (width * height * 3) as usize]; // 3 for RGB

    // Loop through each image
    for img in images {
        // Ensure all images are of the same dimensions
        assert_eq!(img.dimensions(), (width, height));

        // Loop through the width
        for x in 0..width {
            // Loop through the height
            for y in 0..height {
                // Get the pixel at the current position
                let pixel = img.get_pixel(x, y);

                // Accumulate the RGB values
                let index = (y * width + x) as usize * 3;
                sum[index] += pixel[0] as u32;
                sum[index + 1] += pixel[1] as u32;
                sum[index + 2] += pixel[2] as u32;
            }
        }
    }

    // Calculate the average pixel values
    for x in 0..width {
        for y in 0..height {
            let index = (y * width + x) as usize * 3;
            let avg_red = (sum[index] / num_images) as u8;
            let avg_green = (sum[index + 1] / num_images) as u8;
            let avg_blue = (sum[index + 2] / num_images) as u8;

            // Set the average pixel in the new image
            // avg_image.put_pixel(x, y, Rgb([avg_red, avg_green, avg_blue]));
            
            // average the averages
            let avg = (avg_red as u16 + avg_green as u16 + avg_blue as u16) / 3;

            // Then cast back to u8 if needed
            let avg = avg as u8;

            // if a pixel is heavily changed from the two iamges, put it as white on img3
            // else put it as black on img3
            if avg > thresh {
                avg_image.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {    
                avg_image.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }

    // Return the averaged image
    DynamicImage::ImageRgb8(avg_image)
}

pub fn knn(images: Vec<DynamicImage>, thresh: u8) -> DynamicImage {
    // Ensure there is at least one image
    assert!(!images.is_empty());

    let num_images = images.len() as u32;

    // Get the dimensions from the first image
    let (width, height) = images[0].dimensions();
    
    // Create an image to store the foreground mask
    let mut foreground_mask = RgbImage::new(width, height);
    
    // Initialize a vector to store the sum of pixel values
    let mut sum = vec![0u32; (width * height * 3) as usize]; // 3 for RGB

    // Loop through each image
    for img in images {
        // Ensure all images are of the same dimensions
        assert_eq!(img.dimensions(), (width, height));

        // Loop through the width
        for x in 0..width {
            // Loop through the height
            for y in 0..height {
                // Get the pixel at the current position
                let pixel = img.get_pixel(x, y);

                // Accumulate the RGB values
                let index = (y * width + x) as usize * 3;
                sum[index] += pixel[0] as u32;
                sum[index + 1] += pixel[1] as u32;
                sum[index + 2] += pixel[2] as u32;
            }
        }
    }

    // Calculate the average pixel values
    let mut avg_image = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let index = (y * width + x) as usize * 3;
            let avg_red = (sum[index] / num_images) as u8;
            let avg_green = (sum[index + 1] / num_images) as u8;
            let avg_blue = (sum[index + 2] / num_images) as u8;

            // Set the average pixel in the new image
            avg_image.put_pixel(x, y, Rgb([avg_red, avg_green, avg_blue]));
        }
    }

    // Now create the foreground mask
    for x in 0..width {
        for y in 0..height {
            let pixel = avg_image.get_pixel(x, y);
            let foreground_pixel = if (pixel[0] as u32 > thresh as u32) ||
                                      (pixel[1] as u32 > thresh as u32) ||
                                      (pixel[2] as u32 > thresh as u32) {
                Rgb([255, 255, 255]) // Foreground (white)
            } else {
                Rgb([0, 0, 0]) // Background (black)
            };
            foreground_mask.put_pixel(x, y, foreground_pixel);
        }
    }

    // Return the foreground mask
    DynamicImage::ImageRgb8(foreground_mask)
}

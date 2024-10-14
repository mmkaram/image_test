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

            // average the red, green, and blue values individually then average the averages
            let avg_red = (pixel1[0] as u16 + pixel2[0] as u16) / 2;
            let avg_green = (pixel1[1] as u16 + pixel2[1] as u16) / 2;
            let avg_blue = (pixel1[2] as u16 + pixel2[2] as u16) / 2;

            // Then cast back to u8 if needed
            let avg_red = avg_red as u8;
            let avg_green = avg_green as u8;
            let avg_blue = avg_blue as u8;


            // average the averages
            let avg = (avg_red as u16 + avg_green as u16 + avg_blue as u16) / 3;

            // Then cast back to u8 if needed
            let avg = avg as u8;


            // if a pixel is heavily changed from the two iamges, put it as white on img3
            // else put it as black on img3
            if avg > thresh {
                img3.put_pixel(x, y, Rgb([0, 0, 0]));
            } else {    
                img3.put_pixel(x, y, Rgb([255, 255, 255]));
            }
        }
    }

    return DynamicImage::ImageRgb8(img3);
}
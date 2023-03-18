use image::{Rgb, RgbImage};

pub fn draw_rect(width: u32, height: u32, color: Rgb<u8>, padding: u32) -> RgbImage {
    let mut image_buffer = RgbImage::new(width, height);

    let x_bounds = [padding, width - padding];
    let y_bounds = [padding, height - padding];

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        if x >= x_bounds[0] && x <= x_bounds[1] && y >= y_bounds[0] && y <= y_bounds[1] {
            *pixel = color;
        }
    }

    return image_buffer;
}

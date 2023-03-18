use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Point, Scale};

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

pub fn draw_text(
    width: u32,
    height: u32,
    text: &str,
    font_size: u32,
    background: Rgb<u8>,
    color: Rgb<u8>,
) -> RgbImage {
    let mut image_buffer = draw_rect(width, height, background, 0);

    let font_data = include_bytes!("../../assets/Roboto-Regular.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale::uniform(font_size as f32);

    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(
            text,
            scale,
            Point {
                x: 0.0,
                y: 0.0 - v_metrics.ascent,
            },
        )
        .collect();
    let text_width = glyphs
        .iter()
        .map(|g| g.unpositioned().h_metrics().advance_width)
        .sum::<f32>() as u32;
    let text_height = v_metrics.ascent - v_metrics.descent;

    let x = if text_width > width {
        0
    } else {
        (width - text_width) / 2
    };

    let y = ((height as f32 - text_height) / 2.0) as u32;

    draw_text_mut(
        &mut image_buffer,
        color,
        x as i32,
        y as i32,
        scale,
        &font,
        text,
    );

    return image_buffer;
}

use image::Rgb;
use rand::prelude::*;

pub const COLOR_MIN: u8 = 0;
pub const COLOR_MAX: u8 = 255;
pub const DARK_MIN: u8 = 0;
pub const DARK_MAX: u8 = 128;
pub const LIGHT_MIN: u8 = 128;
pub const LIGHT_MAX: u8 = 255;

pub fn random_color() -> Rgb<u8> {
    return random_color_with_range(COLOR_MIN, COLOR_MAX);
}

pub fn random_dark_color() -> Rgb<u8> {
    return random_color_with_range(DARK_MIN, DARK_MAX);
}

pub fn random_light_color() -> Rgb<u8> {
    return random_color_with_range(LIGHT_MIN, LIGHT_MAX);
}

fn random_color_with_range(min: u8, max: u8) -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    return Rgb([
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
        rng.gen_range(min..=max),
    ]);
}

extern crate image;

mod utils {
    pub mod images;
}

use utils::images::draw_rect;
use image::Rgb;
use std::path::PathBuf;

fn main() {
    draw_rect(
        200, 100, Rgb([0, 20, 255]), 25
    ).save(PathBuf::from("./output/rectangle.png")).unwrap();
}

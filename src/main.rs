extern crate image;

mod utils {
    pub mod colors;
    pub mod images;
}

use fake::{
    faker::name::raw::{FirstName, LastName},
    locales::EN,
    Fake,
};
use std::path::PathBuf;
use utils::{
    colors::{random_color, random_dark_color, random_light_color},
    images::{draw_rect, draw_text},
};

fn main() {
    draw_rect(200, 100, random_color(), 25)
        .save(PathBuf::from("./output/rectangle.png"))
        .unwrap();

    let first_name: String = FirstName(EN).fake();
    let last_name: String = LastName(EN).fake();
    let random_name: String = format!("{} {}", first_name, last_name);

    draw_text(
        1500,
        900,
        &random_name,
        200,
        random_dark_color(),
        random_light_color(),
    )
    .save(PathBuf::from("./output/text.png"))
    .unwrap();
}

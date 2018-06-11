extern crate raster;
use raster::ResizeMode;
use raster::PositionMode;
use raster::BlendMode;
use std::cmp::Ordering;

fn generate_steam_logo(logo: &raster::Image, background: &raster::Image) -> raster::Image {
    let mut logo = logo.clone();
    let mut result = background.clone();
    match background.width.cmp(&background.height) {
        Ordering::Less => raster::editor::resize(&mut result, 0, 215, ResizeMode::ExactHeight).unwrap(),
        Ordering::Equal => raster::editor::resize(&mut result, 460, 215, ResizeMode::Exact).unwrap(),
        Ordering::Greater => raster::editor::resize(&mut result, 460, 0, ResizeMode::ExactWidth).unwrap(),
    }
    match logo.width.cmp(&logo.height) {
        Ordering::Less => raster::editor::resize(&mut logo, 0, 215, ResizeMode::ExactHeight).unwrap(),
        Ordering::Equal => raster::editor::resize(&mut logo, 460, 215, ResizeMode::Exact).unwrap(),
        Ordering::Greater => raster::editor::resize(&mut logo, 460, 0, ResizeMode::ExactWidth).unwrap(),
    }

    raster::editor::crop(&mut result, 460, 215, PositionMode::Center, 0, 0).unwrap();
    raster::editor::crop(&mut logo, 460, 215, PositionMode::Center, 0, 0).unwrap();

    raster::editor::blend(&result, &logo, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap()
}

fn main() {
    let logo = raster::open("logo.png").unwrap();
    let background = raster::open("bg.png").unwrap();
    let result_image = generate_steam_logo(&logo, &background);
    raster::save(&result_image, "blank.png").unwrap();
}

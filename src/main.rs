extern crate raster;
use raster::{ResizeMode, PositionMode, BlendMode, Image};
use raster::editor::{crop, blend, resize};
use std::cmp::Ordering;
use std::fs;
use std::path::Path;
use std::process::exit;

fn generate_steam_logo(logo: &Image, background: &Image) -> Image {
    let mut logo = logo.clone();
    let mut result = background.clone();
    match background.width.cmp(&background.height) {
        Ordering::Less => resize(&mut result, 0, 215, ResizeMode::ExactHeight).unwrap(),
        Ordering::Equal => resize(&mut result, 460, 215, ResizeMode::Exact).unwrap(),
        Ordering::Greater => resize(&mut result, 460, 0, ResizeMode::ExactWidth).unwrap(),
    }
    match logo.width.cmp(&logo.height) {
        Ordering::Less => resize(&mut logo, 0, 215, ResizeMode::ExactHeight).unwrap(),
        Ordering::Equal => resize(&mut logo, 460, 215, ResizeMode::Exact).unwrap(),
        Ordering::Greater => resize(&mut logo, 460, 0, ResizeMode::ExactWidth).unwrap(),
    }

    crop(&mut result, 460, 215, PositionMode::Center, 0, 0).unwrap();
    crop(&mut logo, 460, 215, PositionMode::Center, 0, 0).unwrap();

    blend(&result, &logo, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap()
}

fn main() {
    /*
    let logo = raster::open("logo.png").unwrap();
    let background = raster::open("bg.png").unwrap();
    let result_image = generate_steam_logo(&logo, &background);
    raster::save(&result_image, "blank.png").unwrap();
    */
    let mut folders_missing = false;
    println!("Finding backgrounds...");
    let backgrounds_path = Path::new("./backgrounds");
    let backgrounds = fs::read_dir(backgrounds_path);
    if backgrounds.is_err() {
        folders_missing = true;
        println!("Backgrounds folder missing, creating it...");
        fs::create_dir(&backgrounds_path).unwrap();
    }
    let logos_path = Path::new("./logos");
    let logos = fs::read_dir(logos_path);
    if logos.is_err() {
        folders_missing = true;
        println!("Logos folder missing, creating it...");
        fs::create_dir(&logos_path).unwrap();
    }
    let out_path = Path::new("./output");
    let out = fs::read_dir(out_path);
    if out.is_err() {
        println!("Output folder missing, creating it...");
        fs::create_dir(&out_path).unwrap();
    }
    if folders_missing {
        println!("Fill the folders with the images, then rerun this program.");
        exit(1);
    }
    let backgrounds = backgrounds.ok();
    match backgrounds {
        None => {
            println!("The backgrounds folder is empty.");
            exit(1);
        },
        Some(_) => {},
    }
    let mut backgrounds = backgrounds.unwrap();
    let logos = logos.ok();
    match logos {
        None => {
            println!("The logos folder is empty.");
            exit(1);
        },
        Some(_) => {},
    }
    let mut logos = logos.unwrap();
    for background in backgrounds.by_ref() {
        let background_path = background.unwrap().path();
        for logo in logos.by_ref() {
            let logo_path = logo.unwrap().path();
            if background_path != logo_path {
                continue;
            }
            let full_logo_path = logos_path.join(logo_path.as_path()).as_path().to_string_lossy().into_owned();
            let full_background_path = backgrounds_path.join(background_path.as_path()).as_path().to_string_lossy().into_owned();
            let logo_img = raster::open(&full_logo_path).unwrap();
            let background_img = raster::open(&full_background_path).unwrap();
            let result_image = generate_steam_logo(&logo_img, &background_img);
            let full_result_path = out_path.join(logo_path.as_path()).as_path().to_string_lossy().into_owned();
            let save_result = raster::save(&result_image, &full_result_path);
            match save_result {
                Ok(_) => {
                    println!("Successfully generated {}", &full_result_path);
                }
                Err(_) => {
                    println!("ERROR");
                }
            }
        }
    }
}

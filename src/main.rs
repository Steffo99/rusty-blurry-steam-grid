extern crate image;
use std::fs;
use std::path::Path;
use std::process::exit;
use image::imageops;
use image::GenericImage;

fn generate_steam_logo(logo: image::DynamicImage, background: image::DynamicImage) -> image::DynamicImage {
    let resized_background = background.resize_exact(460, 215, image::FilterType::Triangle);
    //Resize the logo
    //15px padding
    let mut resized_logo;
    resized_logo = logo.resize(430, 185, image::FilterType::Triangle);
    //Crop the logo
    resized_logo.crop(0, 0, 430, 185);
    println!("w{} h{}", &resized_logo.width(), &resized_logo.height());
    //Blur the background
    let mut last = resized_background.blur(16.0);
    imageops::overlay(&mut last, &resized_logo, (430 - resized_logo.width()) / 2 + 15, (185 - resized_logo.height()) / 2 + 15);
    last
}

fn main() {
    let mut folders_missing = false;
    println!("Finding backgrounds...");
    let backgrounds_path = Path::new("./backgrounds");
    let backgrounds = fs::read_dir(backgrounds_path);
    if backgrounds.is_err() {
        folders_missing = true;
        println!("Backgrounds folder missing, creating it...");
        fs::create_dir(&backgrounds_path).unwrap();
    }
    println!("Finding logos...");
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
        println!("Found background: {}", background_path.as_path().to_string_lossy().to_mut());
        for logo in logos.by_ref() {
            let logo_path = logo.unwrap().path();
            println!("Found logo: {}", logo_path.as_path().to_string_lossy().to_mut());
            if background_path.file_name().unwrap() != logo_path.file_name().unwrap() {
                println!("Skipping {} and {}", background_path.as_path().to_string_lossy().to_mut(), logo_path.as_path().to_string_lossy().to_mut());
                continue;
            }
            let result_path = Path::new("./output").join(Path::new(logo_path.file_name().unwrap()));
            println!("{} + {} = {}", background_path.as_path().to_string_lossy().to_mut(), logo_path.as_path().to_string_lossy().to_mut(), result_path.as_path().to_string_lossy().to_mut());
            let logo_img = image::open(&logo_path).unwrap();
            let background_img = image::open(&background_path).unwrap();
            let result_img = generate_steam_logo(logo_img, background_img);
            let ref mut result_file = fs::File::create(result_path.as_path()).unwrap();
            result_img.write_to(result_file, image::PNG).unwrap();
            break;
        }
    }
}

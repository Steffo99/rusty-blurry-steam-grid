extern crate image;
use std::fs;
use std::path::PathBuf;
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

fn p_path<'a>(pb: &'a PathBuf) -> &'a str {
    // PathBuf unwrapper
    pb.to_str().expect("Filename contains invalid characters")
}

fn p_filename<'a>(pb: &'a PathBuf) -> &'a str {
    // FileName unwrapper
    pb.file_name().unwrap().to_str().expect("Filename contains invalid characters")
}

fn main() {
    let mut folders_missing = false;
    println!("Finding backgrounds...");
    let backgrounds_path = Path::new("./backgrounds");
    let backgrounds = fs::read_dir(backgrounds_path);
    if backgrounds.is_err() {
        folders_missing = true;
        println!("Backgrounds folder missing, creating it...");
        fs::create_dir(&backgrounds_path).expect("Failed to create backgrounds folder");
    }
    println!("Finding logos...");
    let logos_path = Path::new("./logos");
    let logos = fs::read_dir(logos_path);
    if logos.is_err() {
        folders_missing = true;
        println!("Logos folder missing, creating it...");
        fs::create_dir(&logos_path).expect("Failed to create logos folder");
    }
    let out_path = Path::new("./output");
    let out = fs::read_dir(out_path);
    if out.is_err() {
        println!("Output folder missing, creating it...");
        fs::create_dir(&out_path).expect("Failed to create output folder");
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

    //For every background
    for background in backgrounds.by_ref() {
        //Find its path
        let background_path = background.unwrap().path();
        println!("Found background: {}", p_path(&background_path));

        //Match it to a log
        for logo in logos.by_ref() {
            let logo_path = logo.unwrap().path();
            println!("Found logo: {}", p_path(&logo_path));

            //If the filenames match...
            if p_filename(&background_path) != p_filename(&logo_path) {
                println!("Skipping {} and {}", p_path(&background_path), p_path(&logo_path));
                continue;
            }

            //Create a logo.
            let result_path = Path::new("./output").join(Path::new(logo_path.file_name().unwrap()));
            println!("Created: {}", p_path(&result_path));
            let logo_img = image::open(&logo_path).expect("Failed to open logo image");
            let background_img = image::open(&background_path).expect("Failed to load background image");
            let result_img = generate_steam_logo(logo_img, background_img);
            let ref mut result_file = fs::File::create(result_path).expect("Failed to create the output file");
            result_img.write_to(result_file, image::PNG).expect("Failed to write to the file");
            break;
        }
    }
}

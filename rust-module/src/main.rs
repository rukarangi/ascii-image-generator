extern crate image;

use image::GenericImageView;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

mod filters;

fn main() {
    // --- Take Arguments ---

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let target_path = Path::new(&args[1]);
    let out_path = Path::new(&args[2]);
    let x_modifier = &args[3].parse::<u32>().unwrap();
    let y_modifier = &args[4].parse::<u32>().unwrap();

    // --- Create Access to Output file ---

    // create path
    let path = Path::new(out_path);
    let display = path.display();

    // open file in write-only
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // --- Open Image and get byte Vec ---

    //let img = image::open(Path::new(target_path)).unwrap();
    let img = match image::open(Path::new(target_path)) {
        Err(why) => panic!("Couldn't read image {}: {}", target_path.display(), why),
        Ok(img) => img,
    };

    println!("Dimensions: {:?}", img.dimensions());
    println!("Expected output dimensions (Rough): {} {}", img.dimensions().0 / x_modifier, img.dimensions().1 / y_modifier);

    let mut result_before = String::from("");

    let mut _count = 0;
    for pixel in img.pixels() {
        let (x, y) = (pixel.0, pixel.1);
        let _brightness = pixel.2.0[3];
        let (r, g, b): (u32, u32, u32) = (pixel.2.0[0].into(), pixel.2.0[1].into(), pixel.2.0[2].into());
        let average = (r + g + b) / 3;
        //println!("{:?}", pixel);
        //println!("Pixel {:?}", pixel);
        if y % y_modifier == 0 && x % x_modifier == 0 {
            result_before.push(filters::grayscale_basic(average));
        }
        if x == img.dimensions().0 -1 {
            result_before.push_str("\n");
        }
    }

    // --- Re-Format result to not include blank line ---
    
    let mut result_final = String::from("");
    let mut last_char: char = ' ';

    for c in result_before.chars() {
        if c == last_char && c == '\n' {
            //result_final.push('');
        } else {
            result_final.push(c);
        }
        last_char = c;
    }

    // --- Write final result into output.txt ---

    match file.write_all(result_final.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Wrote File"),
    }
}

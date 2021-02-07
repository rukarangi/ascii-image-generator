static LOREM_IMPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. In \ntristique ultricies semper. Phasellus accumsan ante vulputate lacus \npharetra, at placerat dui cursus. Nullam tortor turpis, varius nec \ntempus sit amet, sagittis quis nisl. Aenean ultrices nisl in nibh \nbibendum molestie. Proin egestas porta arcu, sed suscipit erat \nrutrum at. Nam.";



extern crate image;

use image::GenericImageView;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let grayscale: [&str; 10] = [" ",".",":","-","=","+","*","#","%","@"];

    // let mut ascii_map = HashMap::new();

    // ascii_map.insert(k: K, v: V)

    // --- Take Arguments ---
    let _args: Vec<String> = env::args().collect();
    let mut _target_path = Path::new("dot_test.jpg");
    //target_path = String::from("dot_test.jpg");

    // --- Create Access to Output file ---

    // create path
    let path = Path::new("test.txt");
    let display = path.display();

    // open file in write-only
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // write test lorem to file
    // match file.write_all(LOREM_IMPSUM.as_bytes()) {
    //     Err(why) => panic!("Couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("Wrote File"),
    // }

    // --- Open Image and get byte Vec ---
    let img = image::open(Path::new("data/gradient.png")).unwrap();

    println!("dimensions {:?}", img.dimensions().0);

    let mut result = String::from("");
    let mut _last_line: u32 = 0;

    let mut _count = 0;
    for pixel in img.pixels() {
        let (x, y) = (pixel.0, pixel.1);
        let _brightness = pixel.2.0[3];
        let average = pixel.2.0[0];
        //println!("{:?}", pixel);
        //println!("Pixel {:?}", pixel);
        if y % 3 == 0 && x % 2 == 0 {
            if average < 25 {
                result.push_str(grayscale[0]);
            } else if average < 50 {
                result.push_str(grayscale[1]);
            } else if average < 75 {
                result.push_str(grayscale[2]);
            } else if average < 100 {
                result.push_str(grayscale[3]);
            } else if average < 125 {
                result.push_str(grayscale[4]);
            } else if average < 150 {
                result.push_str(grayscale[5]);
            } else if average < 175 {
                result.push_str(grayscale[6]);
            } else if average < 200 {
                result.push_str(grayscale[7]);
            } else if average < 225 {
                result.push_str(grayscale[8]);
            } else {
                result.push_str(grayscale[9]);
            }
            
        }
        if x == img.dimensions().0 -1 {
            result.push_str("\n");
            // println!("{:?}", pixel);
        }
        
        // count += 1;

        // if count > 20 {
        //     break;
        // }
    }

    match file.write_all(result.as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why),
        Ok(_) => println!("Wrote File"),
    }
}

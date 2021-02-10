extern crate image;

use std::collections::HashMap;
use std::env;

use image::DynamicImage;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1]; // args[0] stands for this command
    let mut img: DynamicImage = image::open(input).unwrap();

    let count = count_colors(&mut img);

    println!("{}", count);
}

fn count_colors(img: &mut DynamicImage) -> f32 {
    let mut dict: HashMap<u32, bool> = HashMap::new();
    let height = img.height();
    let width = img.width();
    let mut pixel_count: usize = 0;
    for (x, y, pixel) in img.pixels() {
        if is_edge(x, y, width, height) {
            let red = pixel[0];
            let green = pixel[1];
            let blue = pixel[2];
            let alpha = pixel[3];
            let key: u32 =
                unsafe { std::mem::transmute::<[u8; 4], u32>([red, green, blue, alpha]).to_be() };

            pixel_count = pixel_count + 1;
            dict.insert(key, true);
        }
    }

    return dict.len() as f32 / pixel_count as f32;
}

fn is_edge(x: u32, y: u32, width: u32, height: u32) -> bool {
    let edge_threshold = 5;
    if x < edge_threshold {
        return true;
    }
    if x > width - edge_threshold {
        return true;
    }
    if y < edge_threshold {
        return true;
    }
    if y > height - edge_threshold {
        return true;
    }

    return false;
}

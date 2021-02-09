extern crate image;

use std::collections::HashMap;
use std::env;

use image::DynamicImage;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1]; // args[0] stands for this command
    println!("{}", input);
    let mut img: DynamicImage = image::open(input).unwrap();

    let count = count_colors(&mut img);

    println!("{}", count);
}

fn count_colors(img: &mut DynamicImage) -> usize {
    let mut dict: HashMap<u32, bool> = HashMap::new();
    for (_, _, pixel) in img.pixels() {
        let red = pixel[0];
        let green = pixel[1];
        let blue = pixel[2];
        let alpha = pixel[3];
        let key: u32 =
            unsafe { std::mem::transmute::<[u8; 4], u32>([red, green, blue, alpha]).to_be() };

        dict.insert(key, true);
    }
    return dict.len();
}

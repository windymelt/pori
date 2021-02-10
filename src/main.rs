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

fn count_colors(img: &mut DynamicImage) -> f32 {
    let mut dict: HashMap<u32, bool> = HashMap::new();
    //let mut hist: HashMap<u16, u32> = HashMap::new();
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
            let idx: u16 = (red as u16 + green as u16 + blue as u16) / 8;
            //match hist.get(&idx) {
            //    Some(&cnt) => hist.insert(idx, cnt + 1),
            //  _ => hist.insert(idx, 1),
            //};
        }
    }

    // let mut past: u32 = 0;
    // let mut sumdiff: u64 = 0;
    // for x in 0..255 {
    //     // TODO sequential
    //     sumdiff += ((hist[&x] / pixel_count as u32) as i32 - past as i32)
    //         .abs()
    //         .pow(2) as u64;
    //     past = hist[&x];
    // }
    // return sumdiff as f32;
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

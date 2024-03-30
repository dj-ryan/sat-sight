// C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-200730_[-0_0].png

use image::{self, GenericImageView, GrayImage};
use std::error::Error;
// use image::{self, imageops::*}

fn count_white_dots(img: &GrayImage) -> Result<u32, Box<dyn Error>> {
    let (image_width, image_height) = img.dimensions();
    println!("width: {}, height: {}", image_width, image_height);
    let mut visited_pixels = vec![vec![false; image_width as usize]; image_height as usize]; // Keep track of visited pixels
    let mut star_count = 0;

    fn dfs(img: &GrayImage, x: u32, y: u32, visited_pixels: &mut Vec<Vec<bool>>, image_width: u32, image_height: u32) {
        if x <= 0
            || y <= 0
            || x >= image_width
            || y >= image_height
            || visited_pixels[y as usize][x as usize] == true
            || img.get_pixel(x, y)[0] < 250
        {
            return;
        }

        visited_pixels[y as usize][x as usize] = true;

        // Explore adjacent white pixels
        dfs(img, x + 1, y, visited_pixels, image_width, image_height);
        dfs(img, x - 1, y, visited_pixels, image_width, image_height);
        dfs(img, x, y + 1, visited_pixels, image_width, image_height);
        dfs(img, x, y - 1, visited_pixels, image_width, image_height);
    }

    for y in 0..image_height {
        for x in 0..image_width {
            if !visited_pixels[y as usize][x as usize] && img.get_pixel(x, y)[0] >= 250 {
                println!("Found white dot x: {}, y: {}", x, y);
                dfs(img, x, y, &mut visited_pixels, image_width, image_height);
                star_count += 1;
            }
        }
    }

    Ok(star_count)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the image (replace with your path)
    let img = image::open(
        //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-200730_[-0_0].png",
        //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-225008_[-23.7499942779541_-34.0000038146973].png",
        "C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-234525_[-23.4999904632568_3.99999928474426].png",
    )?;
    let img = img.grayscale(); // Convert to grayscale
    let img: GrayImage = img.into_luma8();

    // Count white dots
    let dot_count = count_white_dots(&img)?;
    println!("Number of white dots: {}", dot_count);

    Ok(())
}

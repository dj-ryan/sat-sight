// C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-200730_[-0_0].png

use image::{self, GrayImage};
use std::error::Error;
use std::f32;

// use image::{self, imageops::*}

mod star_print;

use crate::star_print::calculate_fingure_print;
use crate::star_print::open_file;
use crate::star_print::parse_star_data;
use crate::star_print::Star;

const IMAGE_SIZE: f32 = 648.0; // image is square
const UNIVERSE_RADIUS: f32 = 10.0; // meters
const ORTHOGONAL_WIDTH: f32 = 3.527; // meters
const VIEWPORT_DEG: f32 = 20.00255; // ((ORTHOGONAL_WIDTH / 2.0) / UNIVERSE_RADIUS).atan().to_degrees() * 2.0;
const PIXEL_DEG: f32 = VIEWPORT_DEG / IMAGE_SIZE;

fn cartesian_to_corrdinates(
    x: u32,
    y: u32,
    image_width: u32,
    image_height: u32,
    viewport_deg: f32,
) -> (f32, f32) {
    let lat = (y as f32 / image_height as f32) * viewport_deg;
    let lon = (x as f32 / image_width as f32) * viewport_deg;
    (lat, lon)
}

fn get_stars_from_image(img: &GrayImage) -> Result<Vec<star_print::Star>, Box<dyn Error>> {
    let (image_width, image_height) = img.dimensions();
    println!("width: {}, height: {}", image_width, image_height);
    let mut visited_pixels = vec![vec![false; image_width as usize]; image_height as usize]; // Keep track of visited pixels
    let mut stars: Vec<star_print::Star> = Vec::new();

    fn dfs(
        img: &GrayImage,
        x: u32,
        y: u32,
        visited_pixels: &mut Vec<Vec<bool>>,
        image_width: u32,
        image_height: u32,
    ) {
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
                println!("Found star at x: {}, y: {}", x, y);
                dfs(img, x, y, &mut visited_pixels, image_width, image_height);
                let (lat, lon) =
                    cartesian_to_corrdinates(x, y, image_width, image_height, VIEWPORT_DEG);
                stars.push(Star {
                    hr: 0,
                    lat: lat,
                    lon: lon,
                    mag: 0.0,
                    fingure_print: 0.0,
                });
                println!("lat: {}, lon: {}", lat, lon);
            }
        }
    }

    Ok(stars)
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
    let stars = get_stars_from_image(&img)?;
    println!("Total number of stars: {}", stars.len());
    let star_with_finger_print = star_print::calculate_fingure_print(stars);
    print!("{:#?}", star_with_finger_print);


    // Calculate fingureprints for stars
    // let csv_file =
    //     open_file("C:/Users/golia/Development/sat-sight/data/star_formated_raw.csv")?;
    // let stars: Vec<Star> = parse_star_data(csv_file)?;
    // let stars_with_fingure_prints = calculate_fingure_print(stars);
    // // print!("{:#?}", stars_with_fingure_prints);

    // let mut writer = csv::Writer::from_path("output.csv")?;
    // for star in stars_with_fingure_prints {
    //     writer.serialize(star)?;
    // }

    Ok(())
}

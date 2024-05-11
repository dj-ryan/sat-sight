//! This module contains the implementation of the `satSight` struct and its methods.
//! The `satSight` struct is used to represent the satellite sighting data.


use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use image::{self, GrayImage};

use std::collections::HashMap;
use serde::Serialize;


pub const IMAGE_SIZE: f32 = 648.0; // image is square
pub const FOV: f32 = 15.0; // Field of view of the camera in degrees

use std::f64::consts::PI;
use std::f32::consts::PI as PI32;


#[derive(Clone, Debug, Serialize)]
pub struct Star {
    pub hr: u32,            // Harvard Revised Number
    pub lat: f32,           // latitude
    pub lon: f32,           // longitude
    pub mag: f32,           // Magnitude
    pub fingure_print: f32, // Hashed fingerprint of the star
}


fn project_astronomical_coords(
    alpha: f64, delta: f64,
    alpha0: f64, delta0: f64,
    scale: f64
) -> (u32, u32) {
    // Convert all angles from degrees to radians for trigonometric functions
    // let alpha = alpha.to_radians();
    // let delta = delta.to_radians();
    // let alpha0 = alpha0.to_radians();
    // let delta0 = delta0.to_radians();
    // //let scale = scale.to_radians();

    // let A = delta.cos() * (alpha - alpha0).cos();
    // let F = scale  * (180.0 / PI) / (delta0.sin() * delta.sin() + A * delta0.cos());

    // let mut LINE = F * (delta0.cos() * delta.sin() - A * delta0.sin());
    // let mut SAMPLE = F * delta.cos() * (alpha - alpha0).sin();

    // LINE += 360.0;
    // SAMPLE += 360.0;
    

    // (LINE as u32, SAMPLE as u32)

    let lambda_a = alpha.to_radians();
    let phi_a = delta.to_radians();
    let lambda_b = alpha0.to_radians();
    let phi_b = delta0.to_radians();

    // let phi_a = alpha.to_radians();
    // let lambda_a = delta.to_radians();
    // let phi_b = alpha0.to_radians();
    // let lambda_b = delta0.to_radians();
    

    //let scale_scale = 100.0 * scale;
    
    let cos_c = (phi_a.sin() * phi_b.sin())
    + (phi_a.cos() * phi_b.cos() * (lambda_b - lambda_a).cos());

    let mut x = scale * (phi_b.cos() * (lambda_b - lambda_a).sin()) / cos_c;
    let mut y = scale * ((phi_a.cos() * phi_b.sin() - phi_a.sin() * phi_b.cos() * (lambda_b - lambda_a).cos()) 
        / cos_c);


    x += 360.0;
    y += 360.0;

    (x as u32, y as u32)
}


pub fn gnomonic_porjection(lambda_view: f64, phi_view: f64, lambda: f64, phi: f64) -> (f64, f64) {

    let cos_c = phi.sin() * phi_view.sin() + phi.cos() * phi_view.cos() * (lambda_view - lambda).cos();

    let x = phi_view.cos() * (lambda_view - lambda).sin() / cos_c;
    
    let y = (phi.cos() * phi_view.sin() - phi.sin() * phi_view.cos() * (lambda_view - lambda).cos()) / cos_c;

    (x, y)
}




/// Calculates the distance between two stars in kilometers
pub fn calculate_distance_between_stars(star1: &Star, star2: &Star) -> f32 {
    let earth_radius = 6371.0; // Radius of the Earth in kilometers

    // Convert the latitude and longitude of the stars to radians
    let lat1_rad = star1.lat.to_radians();
    let lon1_rad = star1.lon.to_radians();
    let lat2_rad = star2.lat.to_radians();
    let lon2_rad = star2.lon.to_radians();

    // Calculate the distance between the two stars
    let delta_lat = lat2_rad - lat1_rad; 
    let delta_lon = lon2_rad - lon1_rad;

    // Haversine formula
    let a = (delta_lat / 2.0).sin().powi(2) 
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let distance = earth_radius * c;

    distance
}


/// Calculates the baring between two stars in degrees
pub fn calculate_baring_between_stars(star1: &Star, star2: &Star) -> f32 {
    // Convert the latitude and longitude of the stars to radians
    let lat1_rad = star1.lat.to_radians();
    let lon1_rad = star1.lon.to_radians();
    let lat2_rad = star2.lat.to_radians();
    let lon2_rad = star2.lon.to_radians();

    // Calculate the baring between the two stars
    let delta_lon = lon2_rad - lon1_rad;

    // Haversine formula
    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();

    let baring = y.atan2(x).to_degrees();

    baring
}

/// Extracts the latitude and longitude tuples from a vector of stars
pub fn extract_lat_lon_tuples(stars: &[Star]) -> Vec<(u32, u32)> {
    stars
        .iter() // Iterate over the stars
        .map(|star| (star.lat as u32, star.lon as u32)) // Create latitude-longitude tuples
        .collect() // Collect into a vector
}



/// Returns the viewable stars from a list of stars based on the field of view and looking direction
/// pub fn get_viewable_stars(fov: f32, window_size: u32, looking_direction: (f32, f32), stars: Vec<Star>) -> Vec<(u32, u32)> {
pub fn get_viewable_stars(fov: f32, window_size: u32, looking_direction: (f32, f32), stars: Vec<Star>) -> Vec<(u32, u32)> {
    let half_fov = fov.to_radians() / 2.0;
    let window_center = (window_size as f32 / 2.0, window_size as f32 / 2.0);

    let looking_direction_rad = (looking_direction.0.to_radians(), looking_direction.1.to_radians());

    stars.into_iter()
        .filter_map(|star| {
            let star_direction = (star.lat.to_radians(), star.lon.to_radians());
            let angle_diff = angle_between_directions(looking_direction_rad, star_direction);

            if angle_diff <= half_fov {
                let (dx, dy) = angle_to_pixel_offset(angle_diff, fov, window_size as f32);
                let (x, y) = (
                    (window_center.0 + dx) as u32,
                    (window_center.1 + dy) as u32,
                );
                println!("Star: {}, x: {}, y: {}", star.hr, x, y);
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}

pub fn angle_between(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
    let phi1 = lat1.to_radians();
    let phi2 = lat2.to_radians();
    let lambda1 = lon1.to_radians();
    let lambda2 = lon2.to_radians();

    let delat_lambda = (lambda2 - lambda1).abs();

    (phi1.sin() * phi2.sin() + phi1.cos() * phi2.cos() * delat_lambda.cos()).acos()
}



pub fn angle_between_stars(star1: Star, star2: Star) -> f32{

    angle_between(star1.lat, star1.lon, star2.lat, star2.lon)
}


pub fn viewable_stars(looking_direction: (f32, f32), stars: Vec<Star>, fov: f32) -> Vec<Star> {
    let half_fov = fov.to_radians() / 2.0;
    //println!("half_fov: {}", half_fov);

    stars.into_iter()
        .filter_map(|star| {
            let angle_diff = angle_between(looking_direction.0, looking_direction.1, star.lat, star.lon);
            if angle_diff <= half_fov {
                //println!("ang: {}", angle_diff);
                //convert_between_angle_and_pixel(fov, 720, looking_direction.0, looking_direction.1, star.latitude, star.longitude);
                println!("Star: {}, lat: {}, lon: {}", star.hr, star.lat, star.lon);
                Some(star)
            } else {
                None
            }
        })
        .collect()
        
    }


pub fn get_pix(stars: Vec<Star>, fov: f32, screen_size: u32, looking_direction: (f32, f32)) -> Vec<(u32, u32)> {
        
        //let scale = screen_size as f32 / fov;
        
        let scale = 2500;

        stars.into_iter()
        .map(|star| 
            //convert_between_angle_and_pixel(fov, screen_size, looking_direction.0, looking_direction.1, star.lat, star.lon)
            
            project_astronomical_coords(
                looking_direction.0 as f64, looking_direction.1 as f64,
                star.lat as f64, star.lon as f64,
                scale as f64
            )

        ).collect()
    }

pub fn convert_between_angle_and_pixel(fov: f32, screen_size: u32, lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> (u32, u32) {
    let screen_center = screen_size as f32 / 2.0;

    let pix_ang = screen_size as f32 / fov;
    // let ang_pix = fov / screen_size as f32;
    //println!("pix_ang: {}, ang_pix: {}", pix_ang, ang_pix);

    //let lat_delta = 180.0 - ((lat1 - lat2).abs() - 180.0).abs();
    //let lon_delta = 180.0 - ((lon1 - lon2).abs() - 180.0).abs();

    let mut lat_delta = 0.0;
    let mut lon_delta = 0.0;

    // let mut lat_delta = lat1 - lat2;
    // let mut lon_delta = lon1 - lon2;  
    
    // // lat_delta = (lat_delta + 180.0) % 360.0 - 180.0; // needs some work, dosn't work for negative long
    // // lon_delta = (lon_delta + 180.0) % 360.0 - 180.0;

    lat_delta = 180.0 - ((lat1 - lat2).abs() - 180.0).abs();
    lon_delta = 180.0 - ((lon1 - lon2).abs() - 180.0).abs();
    

    // lat_delta = angle_between(lat1,0.0, lat2, 0.0);
    // lon_delta = angle_between(0.0, lon1, 0.0, lon2);

    // lat_delta = lat_delta.to_degrees();
    // lon_delta = lon_delta.to_degrees();


    let px_y = lat_delta * pix_ang;
    let px_x = lon_delta * pix_ang;

    //println!("lat_delta: {}, lon_delta: {}", lat_delta, lon_delta);
    //println!("px_y: {}, px_x: {}", px_y, px_x);

    let px_y_shfited = px_y as f32  + screen_center;
    let px_x_shifted = px_x as f32 + screen_center;

    //println!("px_y_shifted: {}, px_x_shifted: {}", px_y_shfited, px_x_shifted);
    //println!("px_y_shifted u32: {}, px_x_shifted u32: {}", px_y_shfited as u32, px_x_shifted as u32);

    (px_y_shfited as u32, px_x_shifted as u32)
}






pub fn angle_between_directions(dir1: (f32, f32), dir2: (f32, f32)) -> f32 {
    let (lat1, lon1) = dir1;
    let (lat2, lon2) = dir2;

    let cos_lat1 = lat1.cos();
    let cos_lat2 = lat2.cos();

    let part1 = (lon2 - lon1).cos() * cos_lat2;
    let part2 = lat2.sin() * cos_lat1 - lat1.sin() * cos_lat2 * (lon2 - lon1).cos();
    part1.atan2(part2).abs()
}

pub fn angle_to_pixel_offset(angle: f32, fov: f32, window_size: f32) -> (f32, f32) {
    let ratio = angle / fov;
    let dx = ratio * window_size / 2.0;
    let dy = dx * (PI32 / 4.0).tan();
    (dx, dy)
}



/// Opens a star file and returns a file object
pub fn open_star_file(file_path: &str) -> Result<File, Box<dyn Error>> {
    let file_path = Path::new(file_path);
    let file = File::open(file_path)?;
    Ok(file)
}

/// Parses a star file and returns a vector of stars
/// | HR | Longitude | Latitude | Magnitude |
/// |----|-----------|----------|-----------|
pub fn parse_star_file(file: File) -> Result<Vec<Star>, Box<dyn Error>> {
    let mut reader = Reader::from_reader(file);

    // Read the CSV records
    let mut stars: Vec<Star> = Vec::new();
    for result in reader.records() {
        let record = result?;
        // Process each record
        let mut star: Star = Star {
            hr: 0,
            lat: 0.0,
            lon: 0.0,
            mag: 0.0,
            fingure_print: 0.0,
        };
        if let Some(hr) = record.get(0) {
            star.hr = hr.parse().unwrap();
        }
        if let Some(lat) = record.get(2) {
            star.lat = lat.parse().unwrap();
        }
        if let Some(lon) = record.get(1) {
            star.lon = lon.parse().unwrap();
        }
        if let Some(mag) = record.get(3) {
            star.mag = mag.parse().unwrap();
        }
        stars.push(star);
    }
    Ok(stars)
}

pub fn increase_contrast(image: &mut GrayImage) {
    // Step 1: Find the minimum and maximum pixel values
    let (min_pixel, max_pixel) = image.iter().fold((u8::MAX, 0), |(min, max), &pixel| {
        (min.min(pixel), max.max(pixel))
    });

    // Step 2: Compute the contrast stretch factor
    let contrast_factor = 255.0 / f64::from(max_pixel - min_pixel);

    // Step 3: Apply the contrast stretch to each pixel
    for pixel in image.iter_mut() {
        // Calculate the new pixel value after applying the contrast stretch
        *pixel = (((*pixel as f64 - f64::from(min_pixel)) * contrast_factor) as u8).min(255);
    }
}


pub fn get_stars_from_image(img: &GrayImage) -> Result<Vec<Star>, Box<dyn Error>> {
    let (image_width, image_height) = img.dimensions();
    //println!("width: {}, height: {}", image_width, image_height);
    let mut visited_pixels = vec![vec![false; image_width as usize]; image_height as usize]; // Keep track of visited pixels
    let mut stars: Vec<Star> = Vec::new();

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
                //println!("Found star at x: {}, y: {}", x, y);
                dfs(img, x, y, &mut visited_pixels, image_width, image_height);
                //let (lat, lon) = cartesian_to_corrdinates(x, y, image_width, image_height, VIEWPORT_DEG);
                stars.push(Star {
                    hr: 0,
                    lat: x as f32,
                    lon: y as f32,
                    mag: 0.0,
                    fingure_print: 0.0,
                });
                //println!("lat: {}, lon: {}", x, y);
            }
        }
    }
    Ok(stars)
}



/// This function takes an image and tests all locations of possibly star locations
/// and returns a sum of the values that it tests
pub fn pin_prick_image(image: &GrayImage, coordinates: &Vec<(u32, u32)>) -> u32 {
    coordinates
        .iter()
        .map(|(x, y)| {
            //println!("x: {}, y: {}", x, y);
            // Handle potential out-of-bounds coordinates
            if *x < image.width() && *y < image.height() {
                image.get_pixel(*x, *y)[0] as u32 // Extract the Luma (grayscale) value
            } else {
                0 // Default value if the coordinate is out-of-bounds
            }
        })
        .sum()
}



pub fn calculate_star_fingure_prints(stars: Vec<Star>) -> Vec<Star> {
    // let mut star_fingure_prints = HashMap::new();
    let mut stars_with_fingure_prints: Vec<Star> = Vec::new();
    for i in 0..stars.len() {
        let mut shortest_constellations = HashMap::new();
        for j in 0..stars.len() {
            if i != j {
                let distance = calculate_distance_between_stars(&stars[i], &stars[j]);
                if shortest_constellations.len() < 5 {
                    shortest_constellations.insert(stars[j].hr, distance);
                } else {
                    let mut max_distance = 0.0;
                    let mut max_distance_star = 0;
                    for (star, dist) in &shortest_constellations {
                        if *dist > max_distance {
                            max_distance = *dist;
                            max_distance_star = *star;
                        }
                    }
                    if distance < max_distance {
                        shortest_constellations.remove(&max_distance_star);
                        shortest_constellations.insert(stars[j].hr, distance);
                    }
                }
            }
        }
        // calculate the baring of the each of the 5 shortest stars

        let mut baring_collection = Vec::new();
        let mut baring_sum = 0.0;
        for (star, _) in &shortest_constellations {
            let baring = calculate_baring_between_stars(&stars[i], &stars[stars.iter().position(|x| x.hr == *star).unwrap()]);
            baring_collection.push(baring+180.0);
            baring_collection.sort_by(|a, b| a.partial_cmp(b).unwrap());
            // calculate the difference between the baring of the stars
            //print!("baring length: {}, ", baring_collection.len());  
        }
        for i in 0..(baring_collection.len() - 1) {
            let difference = baring_collection[i+1] - baring_collection[i];
            //println!("difference: {}", difference);
            baring_sum += difference;
        }
        
        let distance_sum: f32 = shortest_constellations.values().sum();
        println!("Star: {}, print: {}", stars[i].hr, distance_sum + baring_sum);
        // println!("star: {}, shortest_constellations: {:?}", stars[i].hr, shortest_constellations);
        stars_with_fingure_prints.push(Star {
            hr: stars[i].hr,
            lat: stars[i].lat,
            lon: stars[i].lon,
            mag: stars[i].mag,
            fingure_print: baring_sum + distance_sum,
        });

        // let modulo_sum = sum % 1.0; // Assuming you want to take the modulo with 1.0
        // star_fingure_prints.insert(modulo_sum, stars[i].hr);
    }
    // star_fingure_prints
    stars_with_fingure_prints
}


pub fn cartesian_to_corrdinates(
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
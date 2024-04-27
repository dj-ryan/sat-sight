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


#[derive(Clone, Debug, Serialize)]
pub struct Star {
    pub hr: u32,            // Harvard Revised Number
    pub lat: f32,           // latitude
    pub lon: f32,           // longitude
    pub mag: f32,           // Magnitude
    pub fingure_print: f32, // Hashed fingerprint of the star
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

// Helper: Convert (latitude, longitude) to a unit vector
pub fn lat_lon_to_vector(lat_lon: (f32, f32)) -> (f32, f32, f32) {
    let lat_rad = lat_lon.0.to_radians();
    let lon_rad = lat_lon.1.to_radians();

    let x = lat_rad.cos() * lon_rad.cos();
    let y = lat_rad.cos() * lon_rad.sin();
    let z = lat_rad.sin();

    (x, y, z)
}

// Helper: Calculate angle between two 3D vectors
fn angle_between_vectors(v1: (f32, f32, f32), v2: (f32, f32, f32)) -> f32 {
    let dot_product = v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2;
    let magnitudes = (v1.0 * v1.0 + v1.1 * v1.1 + v1.2 * v1.2).sqrt()
        * (v2.0 * v2.0 + v2.1 * v2.1 + v2.2 * v2.2).sqrt();

    (dot_product / magnitudes).acos().to_degrees() 
}

/// Returns the viewable stars from a list of stars based on the field of view and looking direction
pub fn get_viewable_stars(fov: f32, looking_direction: (f32, f32), star_vec: Vec<(f32, f32, f32)>, stars: Vec<Star>) -> Vec<Star> {

    let mut viewable_stars = Vec::new();
    let direction_vector = lat_lon_to_vector(looking_direction);

    let mut i = 0;
    for star in &stars {
        // 1. Convert coordinates to 3D Cartesian vectors
        // let target_star = (star.lat, star.lon);
        // let target_vector = lat_lon_to_vector(target_star);


        // 2. Calculate angle between the vectors
        let angle = angle_between_vectors(direction_vector, star_vec[i]);

        // 3. Compare against half-FOV
        if angle <= (fov / 2.0) {
            viewable_stars.push(star.clone());
        }
        i += 1;
    }

viewable_stars
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
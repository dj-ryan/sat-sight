
use csv::Reader;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use std::collections::HashMap;
use serde::Serialize;

/**
 * // Calculate fingureprints for stars
 * let csv_file = open_file("C:/Users/golia/Development/sat-sight/data/star_formated_raw_short.csv")?;
 * let mut stars: Vec<Star> = parse_star_data(csv_file)?;
 * let stars_with_fingure_prints = calculate_fingure_print(stars);
 * print!("{:#?}", stars_with_fingure_prints);
 */

#[derive(Clone, Debug, Serialize)]
pub struct Star {
    pub hr: u32,            // Harvard Revised Number
    pub lat: f32,           // latitude
    pub lon: f32,           // longitude
    pub mag: f32,           // Magnitude
    pub fingure_print: f32, // Hashed fingerprint of the star
}

pub fn calculate_distance(star1: &Star, star2: &Star) -> f32 {
    let earth_radius = 6371.0; // Radius of the Earth in kilometers

    let lat1_rad = star1.lat.to_radians();
    let lon1_rad = star1.lon.to_radians();
    let lat2_rad = star2.lat.to_radians();
    let lon2_rad = star2.lon.to_radians();

    let delta_lat = lat2_rad - lat1_rad;
    let delta_lon = lon2_rad - lon1_rad;

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let distance = earth_radius * c;

    distance
}

pub fn calculate_baring(star1: &Star, star2: &Star) -> f32 {
    let lat1_rad = star1.lat.to_radians();
    let lon1_rad = star1.lon.to_radians();
    let lat2_rad = star2.lat.to_radians();
    let lon2_rad = star2.lon.to_radians();

    let delta_lon = lon2_rad - lon1_rad;

    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();

    let baring = y.atan2(x).to_degrees();

    baring
}

pub fn open_file(file_path: &str) -> Result<File, Box<dyn Error>> {
    let file_path = Path::new(file_path);
    let file = File::open(file_path)?;
    Ok(file)
}

pub fn parse_star_data(file: File) -> Result<Vec<Star>, Box<dyn Error>> {
    let mut reader = Reader::from_reader(file);

    // Read the CSV records

    let mut stars: Vec<Star> = Vec::new();
    for result in reader.records() {
        let record = result?;
        // Process each record
        // Example: Print the "hr" field of each record
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
        // if let Some(fingure_print) = record.get(6) {
        //     print!("fingure_print: {}, ", fingure_print);
        //     star.fingure_print = fingure_print.parse().unwrap();
        // }
        stars.push(star);
    }
    Ok(stars)
}

pub fn calculate_fingure_print(stars: Vec<Star>) -> Vec<Star> {
    // let mut star_fingure_prints = HashMap::new();
    let mut stars_with_fingure_prints: Vec<Star> = Vec::new();
    for i in 0..stars.len() {
        let mut shortest_constellations = HashMap::new();
        for j in 0..stars.len() {
            if i != j {
                let distance = calculate_distance(&stars[i], &stars[j]);
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
            let baring = calculate_baring(&stars[i], &stars[stars.iter().position(|x| x.hr == *star).unwrap()]);
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

use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;

use std::collections::HashMap;
use std::fmt;

/**
 * Star struct
 */

#[derive(Debug)]
struct Star {
    hr: u32, // Harvard Revised Number
    lat: f32, // latitude
    lon: f32, // longitude
    mag: f32, // Magnitude
    fingure_print: f64 // Hashed fingerprint of the star
}


fn calculate_distance(star1: &Star, star2: &Star) -> f32 {
    let earth_radius = 6371.0; // Radius of the Earth in kilometers

    let lat1_rad = star1.lat.to_radians();
    let lon1_rad = star1.lon.to_radians();
    let lat2_rad = star2.lat.to_radians();
    let lon2_rad = star2.lon.to_radians();

    let delta_lat = lat2_rad - lat1_rad;
    let delta_lon = lon2_rad - lon1_rad;

    let a = (delta_lat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    let distance = earth_radius * c;

    distance
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = Path::new("C:/Users/golia/Development/sat-sight/data/star_formated_raw_short.csv");
    let file = File::open(file_path)?;
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
            fingure_print: 0.0
        };
        if let Some(hr) = record.get(0) {
            star.hr = hr.parse().unwrap();
        }
        if let Some(lat) = record.get(4) {
            star.lat = lat.parse().unwrap();
        }
        if let Some(lon) = record.get(3) {
            star.lon = lon.parse().unwrap();
        }
        if let Some(mag) = record.get(5) {
            star.mag = mag.parse().unwrap();
        }
        // if let Some(fingure_print) = record.get(6) {
        //     print!("fingure_print: {}, ", fingure_print);
        //     star.fingure_print = fingure_print.parse().unwrap();
        // }
        stars.push(star);
    }
    
    //println!("{:?}", stars);



    let mut star_fingure_prints = HashMap::new();
    //for each star in stars find the 5 shortest constellations
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
        println!("Star: {}, Constellations: {:?}", stars[i].hr, shortest_constellations);
        // Sum the distances of the 5 shortest constellations take the modulo of the sum and store it in the hashmap
        let sum: f32 = shortest_constellations.values().sum();
        let modulo_sum = sum % 1.0; // Assuming you want to take the modulo with 1.0
        star_fingure_prints.insert(modulo_sum, stars[i].hr);

    }

    //println!("{:?}", constellations);

    Ok(())
}



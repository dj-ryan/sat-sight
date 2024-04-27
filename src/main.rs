use std::error::Error;
use std::f32;
use image::io::Reader as ImageReader;


mod sat_sight;

use crate::sat_sight::{open_star_file, parse_star_file, Star, get_viewable_stars, pin_prick_image, lat_lon_to_vector, extract_lat_lon_tuples};


fn main() -> Result<(), Box<dyn Error>> {
    
    // ======================================== Open image and and pin prick image and return goodness score

        // Open the image (replace with your path)
    let img_copy = ImageReader::open(
        //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-200730_[-0_0].png",
        //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-225008_[-23.7499942779541_-34.0000038146973].png",
        //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-234525_[-23.4999904632568_3.99999928474426].png",
        //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-04-05-180954_[-11.2500009536743_23.7499980926514].png",
        "C:/Users/golia/Development/sat-sight/data/screenshots/image_00180.png",
        //"C:/Users/golia/Development/sat-sight/data/screenshots/Unsharped_eye.jpg"
    )?
    .decode()?;

    let img_copy = img_copy.grayscale();
    let img_copy = img_copy.blur(5.0);

    let csv_file =
    open_star_file("C:/Users/golia/Development/sat-sight/data/formated/formated_no_nova.csv")?;
    let stars: Vec<Star> = parse_star_file(csv_file)?;
    
    // loop through all veiwing driections using a step of 1 degree 
    // for each direction calculate the viewable stars

    let mut star_vectors = Vec::new();

    for star in stars.clone() {
        let star_vector = lat_lon_to_vector((star.lat, star.lon));
        star_vectors.push(star_vector);
    }


    let mut good_images = Vec::new();
    let mut star_frames = 0;
    for i in -90..90 { // Lambda
        for j in 0..360 { // Phi
            let looking_direction = (i as f32, j as f32);
            let viewable_stars = get_viewable_stars(15.0, looking_direction, star_vectors.clone(), stars.clone());
            if viewable_stars.len() > 0 {
                star_frames += 1;
                // println!("Looking direction: {}, {} - Viewable stars: {:#?}", i, j, viewable_stars.len());
                let star_cords = extract_lat_lon_tuples(&viewable_stars);

                let goodnes_score = pin_prick_image(&img_copy.clone().into_luma8(), &star_cords);
                //println!("Goodness Score: {:#?}", goodnes_score);
                println!("Looking direction: {}, {} - Viewable stars: {:#?} - Goodness Score: {:#?}", i, j, viewable_stars.len(), goodnes_score);

                

                good_images.push((i, j, goodnes_score));
            }
        }
    }

    let mut writer = csv::Writer::from_path("C:/Users/golia/Development/sat-sight/data/calc_output/output_img29_5.csv")?;
    for data_point in good_images {
        writer.serialize(data_point)?;
    }


    println!("Total frames with stars: {:#?}", star_frames);

 
    // ======================================== Open image and compare to other images using fuzzy method
    
    // // Open the image (replace with your path)
    // let img_copy = ImageReader::open(
    //     //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-200730_[-0_0].png",
    //     //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-225008_[-23.7499942779541_-34.0000038146973].png",
    //     //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-03-29-234525_[-23.4999904632568_3.99999928474426].png",
    //     //"C:/Users/golia/Development/sat-sight/data/screenshot_2024-04-05-180954_[-11.2500009536743_23.7499980926514].png",
    //     "C:/Users/golia/Development/sat-sight/data/screenshots/image_00029.png",
    //     //"C:/Users/golia/Development/sat-sight/data/screenshots/Unsharped_eye.jpg"
    // )?
    // .decode()?;

    // let img_copy = img_copy.grayscale();
    // let img_copy = img_copy.blur(5.0);

    // let image_dir = "C:/Users/golia/Development/sat-sight/data/screenshots/images";

    // for entry in fs::read_dir(image_dir).expect("Failed to read directory") {
    //     let entry = entry.expect("Failed to read directory entry");
    //     let path = entry.path();

    //     if path.is_file() && path.extension().unwrap_or_default() == "png" {
    //         // Adjust extension if needed
    //         let image_index = path
    //             .file_stem()
    //             .unwrap()
    //             .to_str()
    //             .unwrap()
    //             .split('_')
    //             .last()
    //             .unwrap()
    //             .parse::<u32>()
    //             .unwrap();

    //         if image_index >= 0 && image_index <= 5 {
    //             //process_image(&path);
    //             print!("{:#?} - ", image_index);
    //             let image = ImageReader::open("C:/Users/golia/Development/sat-sight/data/screenshots/image_00029_shifted.png")?
    //             .decode()?;
    //             let image = image.grayscale();
    //             let star_list = get_stars_from_image(&image.into_luma8())?;
    //             let star_cords = extract_lat_lon_tuples(&star_list);
    //             print!("Total Stars: {:#?} - ", star_list.len());
    //             let goodnes_score = sum_pixel_values(&img_copy.clone().into_luma8(), &star_cords);
    //             println!("Goodness Score: {:#?}", goodnes_score);
    //         }
    //     }
    // }

    //======================================================= Blur and grayscale image
    //img_copy.save("C:/Users/golia/Development/sat-sight/data/screenshots/processed.png")?;

    //img.invert();
    //img.save("C:/Users/golia/Development/sat-sight/data/screenshots/inverted.png")?;
    // let grayscale = img.grayscale();
    // //let img = grayscale.into_luma8();

    // let grayscale_blur = grayscale.blur(5.0);
    // grayscale_blur.save("C:/Users/golia/Development/sat-sight/data/screenshots/blurred.jpg")?;

    // ======================================== Count stars in image
    // let img = img.grayscale(); // Convert to grayscale
    // let img: GrayImage = img.into_luma8();

    // // Count white dots
    // let stars = get_stars_from_image(&img)?;
    // println!("Total number of stars: {}", stars.len());
    // let star_with_finger_print = star_print::calculate_fingure_print(stars);
    //print!("{:#?}", star_with_finger_print);

    // ======================================== Calculate all fingureprints based on star data
    // // Calculate fingureprints for stars
    // let csv_file =
    //     open_file("C:/Users/golia/Development/sat-sight/data/star_formated_raw.csv")?;
    // let stars: Vec<Star> = parse_star_data(csv_file)?;
    // let stars_with_fingure_prints = calculate_fingure_print(stars);
    // // print!("{:#?}", stars_with_fingure_prints);

    // let mut writer = csv::Writer::from_path("./calc/output_long.csv")?;
    // for star in stars_with_fingure_prints {
    //     writer.serialize(star)?;
    // }

    Ok(())
}

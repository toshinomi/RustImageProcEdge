extern crate image;

use image::DynamicImage;
use std::io;
use rust_image_proc_edge::base_module::module::edge_detection;
use rust_image_proc_edge::base_module::base_image_proc::GoImageProc;

fn main() {
    println!("image directory : ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("Failed to read line");
    directory.pop();
    let input_image = directory.to_string() + "/input.jpg";
    let output_image = directory.to_string() + "/output.jpg";
    let mut img: DynamicImage = image::open(input_image).unwrap();

    println!("Image Processing Start!");

    let mut edge = edge_detection::EdgeDetection::new();
    edge.go_image_proc(&mut img);

    img.save(&output_image).unwrap();

    println!("Image Processing End!");
}
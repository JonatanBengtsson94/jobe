use std::{env::current_dir, fs};

use image::GenericImageView;

pub struct Material {}

impl Material {
    pub fn new(filename: &str) {
        let mut filepath = current_dir().expect("Could not find current directory.");
        filepath.push(filename);
        let filepath = fs::read_to_string(filepath).expect("Could not read source code.");
        let bytes = fs::read(filepath).expect("Could not read file.");
        let loaded_image = image::load_from_memory(&bytes).expect("Could not load image.");
        let converted = loaded_image.to_rgb8();
        let size = loaded_image.dimensions();
    }
}

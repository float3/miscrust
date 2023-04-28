use std::env;

use image::{
    imageops, DynamicImage,
};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    //foreach image in folder args[1]
    for entry in std::fs::read_dir(args[2].clone()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap();
        modify_and_safe(path.to_string(),args[1].clone());
    }
}

fn lerp(a: f32, b: f32, c: f32) -> f32 {
    a + (b - a) * c
}

fn modify_and_safe(path: String, logo: String) {
    let mut img = image::open(logo)
        .unwrap()
        .into_rgba32f();

    let mut img2 = image::open(path.clone()).unwrap().into_rgba32f();

    let img2 = imageops::crop(&mut img2, 0, 0, img.width(), img.height());

    let img2 = imageops::resize(
        &img2.to_image(),
        img.width(),
        img.height(),
        imageops::FilterType::Nearest,
    );

    for (x, y, rgb) in img.enumerate_pixels_mut() {
        let mut gray = (rgb[0] + rgb[1] + rgb[2]) / 3.0;

        gray = gray.clamp(0.0, 1.0);

        rgb[0] = lerp(rgb[0], img2.get_pixel(x, y)[0], gray);
        rgb[1] = lerp(rgb[1], img2.get_pixel(x, y)[1], gray);
        rgb[2] = lerp(rgb[2], img2.get_pixel(x, y)[2], gray);
    }
    DynamicImage::from(img)
        .into_rgba16()
        .save(path + "modified.png")
        .unwrap();
}

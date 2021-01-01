// use std::num::Float;

use image::RgbImage;
use image::Rgb;

use crate::shape::{Row, Shape};

pub fn average_image_color(img: &RgbImage) -> Rgb<u8> {
    let mut sum_r: u32 = 0;
    let mut sum_g: u32 = 0;
    let mut sum_b: u32 = 0;
    // 1. replace this with width * height
    let mut count: u32 = 0;

    for &pixel in img.pixels() {
        let [r, g, b] = pixel.0;
        sum_r += r as u32;
        sum_g += g as u32;
        sum_b += b as u32;
        // 1.
        count += 1;
    }
    let avg_r = (sum_r / count) as u8;
    let avg_g = (sum_g / count) as u8;
    let avg_b = (sum_b / count) as u8;
    Rgb([avg_r, avg_g, avg_b])
}

pub fn average_color_in_lines(rows: &Vec<Row>, img: &RgbImage) -> Rgb<u8> {
    let mut sum_r: u32 = 0;
    let mut sum_g: u32 = 0;
    let mut sum_b: u32 = 0;
    let mut count: u32 = 0;
    let (width, height) = img.dimensions();
    let (width, height) = (width as i32, height as i32);
    for &row in rows {   
        let (x1, x2, y) = row.into();
        if y < 0 || y >= height {
            continue;
        }
        let (x1, x2) = (clamp(x1, 0, width-1) as u32, clamp(x2, 0, width-1) as u32);
        let y = y as u32;
        for x in x1..x2+1 {
            let pixel = img.get_pixel(x, y);
            let [r, g, b] = pixel.0;
            sum_r += r as u32;
            sum_g += g as u32;
            sum_b += b as u32;
            count += 1;
        }
    }
    let avg_r = (sum_r as f32 / count as f32).round() as u8;
    let avg_g = (sum_g as f32 / count as f32).round() as u8;
    let avg_b = (sum_b as f32 / count as f32).round() as u8;
    Rgb([avg_r, avg_g, avg_b])
}

pub fn best_color_in_rows(rows: &Vec<Row>, alpha: u8, source: &RgbImage, target: &RgbImage) -> Rgb<u8> {
    let alpha: f32 = alpha as f32 / 255.0;
    let mut sum_r: f32 = 0.0;
    let mut sum_g: f32 = 0.0;
    let mut sum_b: f32 = 0.0;
    let mut count: u32 = 0;
    let (width, height) = source.dimensions();
    let (width, height) = (width as i32, height as i32);
    for &row in rows {   
        let (x1, x2, y) = row.into();
        if y < 0 || y >= height {
            continue;
        }
        let (x1, x2) = (clamp(x1, 0, width-1) as u32, clamp(x2, 0, width-1) as u32);
        let y = y as u32;
        for x in x1..x2+1 {
            let src_pixel = source.get_pixel(x, y);
            let [src_r, src_g, src_b] = src_pixel.0;
            let [src_r, src_g, src_b] = [
                src_r as f32 / 255.0, 
                src_g as f32 / 255.0, 
                src_b as f32 / 255.0
                ];
            // println!("src: ({}, {}, {})", src_r, src_g, src_b);

            let target_pixel = target.get_pixel(x, y);
            let [target_r, target_g, target_b] = target_pixel.0;
            let [target_r, target_g, target_b] = [
                target_r as f32 / 255.0,
                target_g as f32 / 255.0,
                target_b as f32 / 255.0
                ];
            // println!("target: ({}, {}, {})", target_r, target_g, target_b);

            let [cover_r, cover_g, cover_b] = [
                (target_r + src_r * (alpha - 1.0)) / alpha,
                (target_g + src_g * (alpha - 1.0)) / alpha,
                (target_b + src_b * (alpha - 1.0)) / alpha
                ];
            let [cover_r, cover_g, cover_b] = [
                cover_r, 
                cover_g, 
                cover_b
                ];


            sum_r += cover_r;
            sum_g += cover_g;
            sum_b += cover_b;
            count += 1;
        }
    }
    let avg_r = (255.0 * sum_r / count as f32).round() as u8;
    let avg_g = (255.0 * sum_g / count as f32).round() as u8;
    let avg_b = (255.0 * sum_b / count as f32).round() as u8;
    Rgb([avg_r, avg_g, avg_b])
}

pub fn best_color_in_shape(shape: &Box<dyn Shape>, alpha: u8, source: &RgbImage, target: &RgbImage) -> Rgb<u8> {
    let rows = shape.rasterize();
    best_color_in_rows(&rows, alpha, source, target)
}

pub fn mean_square_error(img1: &RgbImage, img2: &RgbImage) -> f32 {
    let mut error: f32 = 0.0;
    assert_eq!(img1.dimensions(), img2.dimensions());
    let count: u32 = img1.dimensions().0 * img1.dimensions().1;

    let mut img1_pixels = img1.pixels();
    let mut img2_pixels = img2.pixels();
    for _ in 0..count {
        let [r1, g1, b1] = (*img1_pixels.next().unwrap()).0;
        let [r2, g2, b2] = (*img2_pixels.next().unwrap()).0;
        let [dr, dg, db] = [
            ((r1 as i32) - (r2 as i32)).pow(2) as f32,
            ((g1 as i32) - (g2 as i32)).pow(2) as f32,
            ((b1 as i32) - (b2 as i32)).pow(2) as f32
        ];
        error += dr + dg + db;
    }
    error = (error/ ((3 * count) as f32)).sqrt();
    // dbg!(error);
    return error;
}

pub fn partial_square_error(error: f32, before: &RgbImage, after: &RgbImage, target: &RgbImage) -> f32 {
    assert_eq!(before.dimensions(), after.dimensions());
    assert_eq!(after.dimensions(), target.dimensions());
    let count: u32 = target.dimensions().0 * target.dimensions().1;
    let mut squared_error: f32 = error.powf(2.0) * (3 * count) as f32;

    let mut before_pixels = before.pixels();
    let mut after_pixels = after.pixels();
    let mut target_pixels = target.pixels();
    for _ in 0..count {
        let [target_r, target_g, target_b] = (*target_pixels.next().unwrap()).0;
        let [before_r, before_g, before_b] = (*before_pixels.next().unwrap()).0;
        let [after_r, after_g, after_b] = (*after_pixels.next().unwrap()).0;
        let [dr, dg, db] = [
            ((target_r as i32) - (before_r as i32)).pow(2) as f32,
            ((target_g as i32) - (before_g as i32)).pow(2) as f32,
            ((target_b as i32) - (before_b as i32)).pow(2) as f32
        ];
        squared_error -= dr + dg + db;
        let [dr, dg, db] = [
            ((target_r as i32) - (after_r as i32)).pow(2) as f32,
            ((target_g as i32) - (after_g as i32)).pow(2) as f32,
            ((target_b as i32) - (after_b as i32)).pow(2) as f32
        ];
        squared_error += dr + dg + db;
    }
    squared_error = (squared_error/ ((3 * count) as f32)).sqrt();
    squared_error
}

pub fn clamp(input: i32, min: i32, max: i32) -> i32 {
    if input < min {
        return min;
    } else if input > max {
        return max;
    } else {
        return input;
    }
}
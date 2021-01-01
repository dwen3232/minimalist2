use std::fmt;

use image::{Rgb, RgbImage};

use rand::Rng;
use rand::rngs::ThreadRng;
use rand::distributions::Uniform;
use rand_distr::{StandardNormal, Distribution};

use crate::shape::{Shape, Rasterizable, Drawable, Mutatable};
use crate::shape::{Row};
use crate::util::{clamp, best_color_in_rows, mean_square_error};
use crate::error;

#[derive(Debug, Default, Clone)]
pub struct Ellipse {
    raster: Option<Vec<Row>>,
    x: i32,
    y: i32,
    x_radius: i32,
    y_radius: i32,
    alpha: u8,
}

impl Ellipse {
    pub fn new(x: i32, y: i32, x_radius: i32, y_radius: i32, alpha: u8) -> Self {
        let mut ellipse = Ellipse {raster: None, x, y, x_radius, y_radius, alpha};
        ellipse.raster = Some(ellipse.new_raster());
        ellipse
    }
    pub fn random(dimensions: (u32, u32), mut rng: &mut ThreadRng) -> Self {
        let (width, height) = (dimensions.0 as i32, dimensions.1 as i32);
        let (x_distr, y_distr) = (Uniform::new(0, width), Uniform::new(0, height));
        let x = x_distr.sample(&mut rng);
        let y = y_distr.sample(&mut rng);
        let x_radius = x_distr.sample(&mut rng);
        let y_radius = y_distr.sample(&mut rng);
        Ellipse::new(x, y, x_radius, y_radius, 128)
    }
}

impl Shape for Ellipse {
    fn error(&self, source: &RgbImage, target: &RgbImage) -> f32 {
        
        let mut source = source.clone();
        self.draw_best_color(&mut source, &target);
        // let best_color = best_color_in_rows(&self.rasterize(), self.alpha, &source, &target);
        // self.draw_to_image(&mut source, best_color, self.alpha);
        mean_square_error(&source, &target)
    }
}

impl Mutatable for Ellipse {
    fn mutate(&mut self, dimensions: (u32, u32), mut rng: &mut ThreadRng) {
        let rate = 4.0;
        let (width, height) = (dimensions.0 as i32, dimensions.1 as i32);
        match Uniform::new(0, 4).sample(&mut rng) {
            0 => {
                // println!("x mutate");
                let delta = (rate * rng.sample::<f32, _>(StandardNormal)).round() as i32;
                // println!("delta: {}", delta);
                let new_x = self.x + delta;
                self.x = clamp(new_x, 0, width-1);
            },
            1 => {
                // println!("y mutate");
                let delta = (rate * rng.sample::<f32, _>(StandardNormal)).round() as i32;
                // println!("delta: {}", delta);
                let new_y = self.y + delta;
                self.y = clamp(new_y, 0, height-1);
            },
            2 => {
                // println!("x_rad mutate");
                let delta = (rate * rng.sample::<f32, _>(StandardNormal)).round() as i32;
                // println!("delta: {}", delta);
                let new_x_radius = self.x_radius + delta;
                self.x_radius = clamp(new_x_radius, 0, width-1);
            },
            3 => {
                // println!("y_rad mutate");
                let delta = (rate * rng.sample::<f32, _>(StandardNormal)).round() as i32;
                // println!("delta: {}", delta);
                let new_y_radius = self.y_radius + delta;
                self.y_radius = clamp(new_y_radius, 0, height-1);
            },
            _ => {
                println!("do nothing lmao");
                // self.mutate(dimensions, rng);
            }
        }
        self.raster = Some(self.new_raster());
    }
}

impl Drawable for Ellipse {
    fn best_color(&self, source: &RgbImage, target: &RgbImage) -> Rgb<u8> {
        best_color_in_rows(&self.rasterize(), self.alpha, &source, &target)
    }

    fn draw_best_color(&self, mut source: &mut RgbImage, target: &RgbImage) {
        let best_color = self.best_color(&source, &target);
        self.draw_to_image(&mut source, best_color, self.alpha);
    }
}

impl Rasterizable for Ellipse {
    fn rasterize(&self) -> Vec<Row> {
        self.raster.as_ref().expect("raster was not created").to_vec()
    }

    fn new_raster(&self) -> Vec<Row> {
        let mut rows = Vec::new();
        let (a, b) = (self.x_radius, self.y_radius);
        let (x_c, y_c) = (self.x, self.y);
        let b2: i32 = b.pow(2);
        let ratio = a as f64 / b as f64;
        rows.push(Row::new(x_c - a, x_c + a, y_c));
        let mut y: i32 = 1;
        while y < b {
            let x = (((b2 - y*y) as f64).sqrt() * ratio).round() as i32;
            let (x1, x2) = (x_c - x, x_c + x);
            let (y1, y2) = (y_c - y, y_c + y);
            if y_c >= y {
                rows.push(Row::new(x1, x2, y1));
            }
            rows.push(Row::new(x1, x2, y2));
            y += 1;
        }
        // rows.sort();
        rows
    }
}

// For printing Ellipse information!
impl fmt::Display for Ellipse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ellipse centered at ({}, {}) with radii ({}, {})", self.x, self.y, self.x_radius, self.y_radius)
    }
}
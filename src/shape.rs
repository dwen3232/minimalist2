use std::fmt::{Debug, Display};

use image::{Rgb, RgbImage};
use rand::{Rng, rngs::ThreadRng};
use dyn_clone::DynClone;

use crate::error;
use crate::util::{clamp};

mod ellipse;
mod row;

pub use ellipse::Ellipse;
pub use row::Row;

// currently, only Ellipse shape.  Maybe add rotated ellipse, polygon when done?
pub enum ShapeKind {
    Ellipse,
}

impl ShapeKind {
    pub fn random(&self, dimensions: (u32, u32), mut rng: &mut ThreadRng) -> Box<dyn Shape> {
        match self {
            Self::Ellipse => Box::new(Ellipse::random(dimensions, &mut rng))
        }
    }
}

// TRAITS
pub trait Shape: Mutatable + Drawable + DynClone + Display {
    fn error(&self, source: &RgbImage, target: &RgbImage) -> f32;
}

pub trait Mutatable {
    fn mutate(&mut self, dimensions: (u32, u32), rng: &mut ThreadRng);
}

pub trait Drawable: Rasterizable {
    fn best_color(&self, source: &RgbImage, target: &RgbImage) -> Rgb<u8>;
    fn draw_best_color(&self, source: &mut RgbImage, target: &RgbImage);
    fn draw_to_image(&self, img: &mut RgbImage, color: Rgb<u8>, alpha: u8) {
        let rows = self.rasterize();
        let alpha: f32 = alpha as f32 / 255.0;
        let (width, height) = img.dimensions();
        let (width, height) = (width as i32, height as i32);
        for row in rows {
            let (x1, x2, y) = row.into();
            if y < 0 || y >= height {
                continue;
            }
            let (x1, x2) = (clamp(x1, 0, width-1) as u32, clamp(x2, 0, width-1) as u32);
            let y = y as u32;
            for x in x1..x2+1 {
                let pixel = img.get_pixel(x, y);
                let [img_r, img_g, img_b] = pixel.0;
                let [r, g, b] = color.0;

                let [new_r, new_g, new_b] = [
                    (r as f32 * alpha + img_r as f32 * (1.0  - alpha)).round() as i32,
                    (g as f32 * alpha + img_g as f32 * (1.0 - alpha)).round() as i32,
                    (b as f32 * alpha + img_b as f32 * (1.0 - alpha)).round() as i32
                    ];
                // dbg!(new_r);
                let [new_r, new_g, new_b] = [
                    clamp(new_r, 0, 255) as u8,
                    clamp(new_g, 0, 255) as u8,
                    clamp(new_b, 0, 255) as u8
                    ];
                // dbg!(new_r, new_g, new_b);
                img.put_pixel(x, y, Rgb([new_r, new_g, new_b]));
            }
        }
    }
}

pub trait Rasterizable {
    fn rasterize(&self) -> Vec<Row>;
    fn new_raster(&self) -> Vec<Row>;
}

// STRUCTS



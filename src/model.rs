use std::path::Path;

use image::Rgb;
use image::RgbImage;
use image::ImageBuffer;

use rand::{Rng, rngs::ThreadRng};

use dyn_clone::{clone_box};

use crate::shape::{self, Shape, ShapeKind};
use crate::optimize;
use crate::util;
use crate::error;

pub struct Model {
    pub background: Rgb<u8>,
    current_img: RgbImage,
    target_img: RgbImage,
    pub size: (u32, u32),

    pub shapes: Vec<Box<dyn Shape>>,
    pub errors: Vec<f32>,
    // add tracking vectors for shapes, colors, scores
    // add multithreading?
}

impl Model {
    pub fn new<P: AsRef<Path>>(path: P) -> error::Result<Self> {
        let target_img = image::open(path)?.into_rgb8();
        let size = target_img.dimensions();
        let background = util::average_image_color(&target_img);
        let current_img = ImageBuffer::from_fn(size.0, size.1, |_x, _y| {
            background
        });
        let shapes = Vec::new();
        let errors = vec![util::mean_square_error(&current_img, &target_img)];

        let model = Model {
            background,
            current_img,
            target_img,
            size,
            shapes,
            errors,
        };
        Ok(model)
    }
    pub fn step(&mut self, kind: &ShapeKind, num_climbs: u32, max_age: u32, num_rand: u32) {
        let (shape, error) = self.next_shape(kind, num_climbs, max_age, num_rand);
        shape.draw_best_color(&mut self.current_img, &self.target_img);
        self.shapes.push(shape);
        self.errors.push(error);

    }

    fn next_shape(&mut self, kind: &ShapeKind, num_climbs: u32, max_age: u32, num_rand: u32) -> (Box<dyn Shape>, f32) {
        let mut rng = rand::thread_rng();
        optimize::best_random_hill_climb(kind, num_climbs, max_age, num_rand, &self.current_img, &self.target_img, &mut rng)
    }

    // FOR TESTING PURPOSES
    pub fn save_current_img<P: AsRef<Path>>(&self, path: P) -> error::Result<()> {
        self.current_img.save(path)?;
        Ok(())
    }
}
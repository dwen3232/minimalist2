use image::RgbImage;
use rand::{Rng, rngs::ThreadRng};
use dyn_clone::{clone_box};

use crate::shape::{Shape, ShapeKind};
use crate::shape::{Rasterizable, Drawable, Mutatable};
use crate::shape::{Ellipse};

use crate::util::{best_color_in_shape};

pub fn best_random_shape(kind: &ShapeKind, num_rand: u32, source: &RgbImage, target: &RgbImage, mut rng: &mut ThreadRng) -> (Box<dyn Shape>, f32) {
    let dimensions = source.dimensions();
    let mut shape: Box<dyn Shape> = kind.random(dimensions, &mut rng);
    let mut error: f32 = shape.error(&source, &target);
    for _ in 1..num_rand {
        let new_shape = kind.random(dimensions, &mut rng);
        let new_error = new_shape.error(&source, &target);
        if new_error < error {
            shape = new_shape;
            error = new_error;
        }
    }
    println!("best random shape: {}, {}", shape, error);
    (shape, error)
}

pub fn hill_climb(init_shape: Box<dyn Shape>, init_error: f32, max_age: u32, source: &RgbImage, target: &RgbImage, mut rng: &mut ThreadRng) -> (Box<dyn Shape>, f32) {
    let dimensions = source.dimensions();
    let mut shape = init_shape;
    let mut error = init_error;
    let mut age = 0;
    let mut steps = 0;
    while age < max_age {
        // println!("current age: {}", age);
        let mut new_shape = clone_box(&*shape);
        new_shape.mutate(dimensions, &mut rng);
        let new_error = new_shape.error(&source, &target);
        // println!("new_error: {}", new_error);
        if new_error < error {
            shape = new_shape;
            error = new_error;
            age = 0;
        } else {
            age += 1;
        }
        steps += 1;
    }
    println!("steps: {}", steps);
    println!("hill climb: {}, {}", shape, error);
    (shape, error)
}

pub fn best_hill_climb(
    init_shape: Box<dyn Shape>, init_error: f32,
    num_climbs: u32, max_age: u32, 
    source: &RgbImage, target: &RgbImage,
    mut rng: &mut ThreadRng
) -> (Box<dyn Shape>, f32) {
    let mut shape = clone_box(&*init_shape);
    let mut error = init_error;
    for _ in 0..num_climbs {
        let (new_shape, new_error) = hill_climb(clone_box(&*init_shape), init_error, max_age, &source, &target, &mut rng);
        if new_error < error {
            shape = new_shape;
            error = new_error;
        }
    }
    println!("best hill climb: {}, {}", shape, error);
    (shape, error)
}

pub fn best_random_hill_climb(
    kind: &ShapeKind,
    num_climbs: u32, max_age: u32, num_rand: u32,
    source: &RgbImage, target: &RgbImage,
    mut rng: &mut ThreadRng
) -> (Box<dyn Shape>, f32) {
    let (init_shape, init_error) = best_random_shape(&kind, num_rand, &source, &target, &mut rng);
    let (mut shape, mut error) = hill_climb(init_shape, init_error, max_age, &source, &target, &mut rng);
    for _ in 1..num_climbs {
        let (init_shape, init_error) = best_random_shape(&kind, num_rand, &source, &target, &mut rng);
        let (new_shape, new_error) =  hill_climb(init_shape, init_error, max_age, &source, &target, &mut rng);
        if new_error < error {
            shape = new_shape;
            error = new_error;
        }
    }
    println!("best hill climb: {}, {}", shape, error);
    (shape, error)
}
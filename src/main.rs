#![allow(dead_code)]
#![allow(unused_imports)]
extern crate image;
extern crate rand;
extern crate rand_distr;
extern crate dyn_clone;

mod shape;
mod model;
mod optimize;
mod util;
mod error;

use image::{Rgb, RgbImage, ImageBuffer};
use rand::Rng;

use shape::Ellipse;
use shape::{ShapeKind, Shape};
use shape::{Drawable, Mutatable, Rasterizable};
use model::Model;

fn main() {
    let num_shapes = 50;
    let kind = ShapeKind::Ellipse;
    let num_climbs = 4;
    let max_age = 100;
    let num_rand = 1000;
    let mut step_counter = 0;
    let mut model = Model::new("data/mona.jpg").expect("Failed to open path");
    while step_counter <= num_shapes {
        step_counter += 1;
        model.step(&kind, num_climbs, max_age, num_rand);
        if step_counter % 5 == 0 {
            model.save_current_img(format!("data/mona_{}.png", step_counter)).expect("Failed to save to path");
        }
    }
    model.save_current_img("data/mona_final.png").expect("Failed to save to path");
}

fn test_hill_climb() {
    let mut rng = rand::thread_rng();
    let kind = ShapeKind::Ellipse;
    let num_rand = 1000;
    let max_age = 100;
    let target_img = image::open("data/mona.jpg").expect("opening target").into_rgb8();
    let size = target_img.dimensions();
    let background = util::average_image_color(&target_img);
    let current_img = ImageBuffer::from_fn(size.0, size.1, |_x, _y| {
        background
    });
    let init_error = util::mean_square_error(&current_img, &target_img);
    println!("init_error: {}", init_error);
    let (shape, error) = optimize::best_random_shape(&kind, num_rand, &current_img, &target_img, &mut rng);
    println!("{}", shape);
    println!("{}", error);
    let (shape, error) = optimize::hill_climb(shape, error, max_age, &current_img, &target_img, &mut rng);
    println!("{}", shape);
    println!("{}", error);
}

fn test_best_random_hill_climb() {
    let mut rng = rand::thread_rng();
    let kind = ShapeKind::Ellipse;
    let num_rand = 1000;
    let max_age = 100;
    let num_climbs = 4;

    let target_img = image::open("data/mona.jpg").expect("opening target").into_rgb8();
    let size = target_img.dimensions();
    let background = util::average_image_color(&target_img);
    let current_img = ImageBuffer::from_fn(size.0, size.1, |_x, _y| {
        background
    });

    let init_error = util::mean_square_error(&current_img, &target_img);
    println!("init_error: {}", init_error);

    let (shape, error) = optimize::best_random_hill_climb(&kind, max_age, num_climbs, num_rand, &current_img, &target_img, &mut rng);
    println!("{}", shape);
    println!("{}", error);
}

fn test_draw() {
    let mut rng = rand::thread_rng();
    let mut img1 = image::ImageBuffer::from_fn(512, 512, |_x, _y| {
        image::Rgb([0u8,0u8,0u8])
    });
    let pixel = img1.get_pixel(512/2, 512/2);
    println!("source image: {:?}", pixel);

    let img2 = image::ImageBuffer::from_fn(512, 512, |_x, _y| {
        image::Rgb([100u8,100u8,80u8])
    });
    let pixel = img2.get_pixel(512/2, 512/2);
    println!("target image: {:?}", pixel);

    let mut ellipse1 = Ellipse::new(512/2, 0, 512/2, 512/2, 128);
    println!("{}", ellipse1);
    ellipse1.mutate((512, 512), &mut rng);
    println!("{}", ellipse1);
    let ellipse2 = Ellipse::new(512/2, 511, 512/2, 512/2, 128);

    let lines = shape::Row::full_image(512, 512);
    let best_color = util::best_color_in_rows(&lines, 128, &img1, &img2);
    println!("best color: {:?}", best_color);

    ellipse1.draw_to_image(&mut img1, best_color, 128);
    ellipse2.draw_to_image(&mut img1, best_color, 128);

    let pixel = img1.get_pixel(512/2, 512/2);
    println!("result: {:?}", pixel);
    img1.save("data/test.png").expect("");
    img1.save("data/test.jpeg").expect("");
}
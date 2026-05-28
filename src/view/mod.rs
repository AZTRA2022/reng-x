pub mod bar;
pub mod file;
pub mod view;
pub use bar::*;

pub use file::*;
pub use view::*;
extern crate rand;
pub use csv;
use macroquad::color::Color;
use macroquad::prelude::*;
use rand::RngExt;
use std::f32::consts::PI;
use std::fs::*;

#[derive(Debug, Clone, Copy)]
pub struct Dim {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub enum FileExt<'a> {
    CSV(&'a str, i32),
    TXT(&'a str),
}

pub fn gen_array(length: usize) -> Vec<f32> {
    let mut rng = rand::rng();

    (0..length)
        .map(|_| rng.random_range(10.0..=255.0))
        .collect()
}

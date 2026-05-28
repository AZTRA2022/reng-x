use std::ops::Mul;

use macroquad::{miniquad::BufferUsage::Stream, prelude::*, telemetry::frame};
extern crate rand;
use rand::{RngExt, fill};

fn gen_array(length: Option<usize>) -> Vec<f32> {
    let mut arr: Vec<f32> = Vec::with_capacity(length.unwrap());
    for _ in 0..length.unwrap() {
        arr.push(rand::random_range(0.0..=550.0));
    }
    arr
}

fn draw_bar() {}

#[derive(Debug, Clone, Copy)]
struct Dim {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
struct Bar {
    pub color: Color,
    pub dim: Dim,
    pub value: f32,
}

#[derive(Debug, Clone)]
struct Plane {
    pub data: Vec<f32>,
    pub bars: Option<Vec<Bar>>,
    pub sw: f32,
    pub sh: f32,
    pub gradient: f32,
    pub print_data: bool,
    pub index: bool,
}

impl Plane {
    fn draw_bars(&mut self) {
        for bar in self.clone().bars.unwrap() {
            draw_rectangle(
                bar.dim.x,
                bar.dim.y,
                bar.dim.width,
                bar.dim.height,
                bar.color,
            );
            if self.index {
                draw_text(
                    format!("{}", bar.value),
                    bar.dim.x,
                    bar.dim.y - 10.0,
                    12.0,
                    Self::colorizer(bar.value),
                );
            }
            if self.print_data {
                draw_text(
                    format!("{:?}", &self.data.clone()[0..=9]),
                    10.0,
                    10.0,
                    16.0,
                    Self::colorizer(self.data[0]),
                );
            }
        }
        Self::refresh(self);
    }

    fn colorizer(value: f32) -> Color {
        match value {
            v if v < 25.0 => DARKBLUE,
            v if v < 50.0 => BLUE,
            v if v < 75.0 => SKYBLUE,
            v if v < 100.0 => DARKGREEN,
            v if v < 125.0 => GREEN,
            v if v < 150.0 => LIME,
            v if v < 175.0 => DARKPURPLE,
            v if v < 200.0 => PURPLE,
            v if v < 225.0 => VIOLET,
            v if v < 250.0 => MAGENTA,
            v if v < 275.0 => PINK,
            v if v < 300.0 => BEIGE,
            v if v < 325.0 => LIGHTGRAY,
            v if v < 350.0 => GRAY,
            v if v < 375.0 => DARKGRAY,
            v if v < 400.0 => BROWN,
            v if v < 425.0 => DARKBROWN,
            v if v < 450.0 => YELLOW,
            v if v < 475.0 => GOLD,
            v if v < 500.0 => ORANGE,
            v if v < 525.0 => MAROON,
            _ => RED,
        }
    }

    fn init(&mut self) {
        let data_size = self.data.len();
        let mut n = 0;
        for item in self.data.clone() {
            let _x = n as f32 * (self.sw / data_size as f32);
            let _y = self.sh - (item / 2.0);
            let bar_dim = Dim {
                x: _x,
                y: _y,
                width: self.sw / data_size as f32,
                height: (item),
            };
            // dbg!(self.gradient.clone());
            let bar = Bar {
                dim: bar_dim,
                color: Self::colorizer(item),
                value: item,
            };
            let bars = self.bars.as_mut().unwrap();
            self.gradient = (self.sh + self.sw) * 5.0;
            bars.push(bar);
            n += 1;
        }
    }
    fn refresh(&mut self) {
        self.sw = screen_width();
        self.sh = screen_height();
        self.bars.as_mut().unwrap().clear();
    }
}

#[macroquad::main("Diagram")]
async fn main() {
    let array = gen_array(Some(50));
    dbg!(array.clone());

    let mut plane = Plane {
        data: array,
        bars: Some(Vec::new()),
        sw: 0.0,
        sh: 0.0,
        gradient: 0.0,
        print_data: true,
        index: true,
    };
    loop {
        clear_background(BLACK);
        plane.init();
        plane.draw_bars();
        next_frame().await;
    }
}

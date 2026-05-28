use std::f32::consts::PI;

use macroquad::prelude::*;
extern crate rand;
use rand::RngExt;

#[derive(Debug, Clone, Copy)]
struct Dim {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, Copy)]
struct Bar {
    color: Color,
    dim: Dim,
    value: f32,
}

#[derive(Debug)]
struct Plane {
    data: Vec<f32>,
    bars: Vec<Bar>,
    sw: f32,
    sh: f32,
    print_data: bool,
    index: bool,
}

impl Plane {
    fn colorizer(value: f32) -> Color {
        match value {
            v if v < 50.0 => BLUE,
            v if v < 100.0 => GREEN,
            v if v < 150.0 => YELLOW,
            v if v < 200.0 => ORANGE,
            _ => RED,
        }
    }

    fn rebuild_bars(&mut self) {
        self.sw = screen_width();
        self.sh = screen_height();
        self.bars.clear();

        let data_size = self.data.len();
        let max_value = self.data.iter().copied().fold(1.0_f32, f32::max);

        let index_margin = if self.index { 80.0 } else { 0.0 };
        let available_height = self.sh - index_margin;

        let bar_width = self.sw / data_size as f32;

        for (n, &item) in self.data.iter().enumerate() {
            let bar_height = (item / max_value) * available_height;

            let dim = Dim {
                x: n as f32 * bar_width,
                y: self.sh - bar_height,
                width: bar_width,
                height: bar_height,
            };

            self.bars.push(Bar {
                dim,
                color: Self::colorizer(item),
                value: item,
            });
        }
    }

    fn draw(&self) {
        for bar in &self.bars {
            draw_rectangle(
                bar.dim.x,
                bar.dim.y,
                bar.dim.width,
                bar.dim.height,
                bar.color,
            );

            if self.index {
                draw_text_ex(
                    &format!("{:.1}", bar.value),
                    bar.dim.x + bar.dim.width / 3.0,
                    bar.dim.y - 5.0,
                    TextParams {
                        font_size: 13,
                        rotation: PI / 2.0,
                        color: WHITE,
                        ..Default::default()
                    },
                );
            }
        }

        if self.print_data {
            let preview = &self.data[..self.data.len().min(10)];

            draw_text(&format!("{:?}", preview), 10.0, 20.0, 20.0, WHITE);
        }
    }

    #[allow(unused)]
    fn mutate_array(&mut self) {
        let mut rng = rand::rng();

        for item in &mut self.data {
            let variation = rng.random_range(-2.0..=2.0);

            *item += variation;

            *item = item.clamp(10.0, 500.0);
        }
    }
}

fn gen_array(length: usize) -> Vec<f32> {
    let mut rng = rand::rng();

    (0..length)
        .map(|_| rng.random_range(10.0..=255.0))
        .collect()
}

#[macroquad::main("Diagram")]
async fn main() {
    let mut plane = Plane {
        data: gen_array(25),
        bars: Vec::with_capacity(25),
        sw: 0.0,
        sh: 0.0,
        print_data: true,
        index: true,
    };

    loop {
        clear_background(BLACK);

        plane.rebuild_bars();

        plane.draw();

        plane.mutate_array();

        next_frame().await;
        draw_fps();
    }
}

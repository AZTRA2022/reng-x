use super::*;

#[derive(Debug)]
pub struct View {
    pub data: Vec<f32>,
    pub bars: Vec<Bar>,
    pub sw: f32,
    pub sh: f32,
    pub print_data: bool,
    pub index: bool,
}

impl View {
    pub fn colorizer(value: f32) -> Color {
        match value {
            v if v < 50.0 => BLUE,
            v if v < 100.0 => GREEN,
            v if v < 150.0 => YELLOW,
            v if v < 200.0 => ORANGE,
            _ => RED,
        }
    }

    pub fn rebuild_bars(&mut self) {
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

    pub fn draw(&self) {
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
    pub fn mutate_array(&mut self) {
        let mut rng = rand::rng();

        for item in &mut self.data {
            let variation = rng.random_range(-2.0..=2.0);

            *item += variation;

            *item = item.clamp(10.0, 500.0);
        }
    }
}

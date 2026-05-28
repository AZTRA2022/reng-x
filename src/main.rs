use clap::Parser;
use macroquad::prelude::*;
pub mod view;
use view::*;

#[derive(Parser, Debug)]
#[command(name = "reng-x", about = "Visualiseur de données en barres")]
struct Args {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    column: i32,

    #[arg(short, long)]
    random: bool,

    #[arg(short = 'n', long, default_value_t = 25)]
    count: usize,

    #[arg(long)]
    no_index: bool,

    #[arg(long)]
    no_print: bool,
}

#[macroquad::main("Diagram")]
async fn main() {
    let args = Args::parse();

    let data = if args.random || args.file.is_none() {
        gen_array(args.count)
    } else {
        let path = args.file.as_deref().unwrap();
        if path.ends_with(".csv") {
            read_from::<f32>(&FileExt::CSV(path, args.column))
        } else {
            read_from::<f32>(&FileExt::TXT(path))
        }
    };

    let mut plane = View {
        bars: Vec::with_capacity(data.len()),
        sw: 0.0,
        sh: 0.0,
        print_data: !args.no_print,
        index: !args.no_index,
        data,
    };

    loop {
        clear_background(BLACK);
        plane.rebuild_bars();
        plane.draw();
        plane.mutate_array();
        next_frame().await;
    }
}

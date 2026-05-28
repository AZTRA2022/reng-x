use macroquad::prelude::*;
pub mod view;
use view::*;
#[allow(unused)]
const TXT_TEST_FILE: &'static str = "/home/aztra/codes/reng-x/src/test.txt";
const CSV_TEST_FILE: &'static str = "/home/aztra/Bureau/test.csv";

#[macroquad::main("Diagram")]
async fn main() {
    let data_1 = read_from::<f32>(&FileExt::CSV(CSV_TEST_FILE, 1));
    dbg!(&data_1);
    let data_2 = gen_array(25);
    let mut plane = View {
        data: data_1,
        bars: Vec::with_capacity(25),
        sw: 0.0,
        sh: 0.0,
        print_data: true,
        index: true,
    };
    // let file = File::open("test.txt");
    loop {
        clear_background(BLACK);
        plane.rebuild_bars();
        plane.draw();
        plane.mutate_array();
        next_frame().await;
    }
}

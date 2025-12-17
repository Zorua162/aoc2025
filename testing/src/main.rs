use macroquad::prelude::*;

#[macroquad::main("Display")]
async fn main() {
    loop {
        clear_background(WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        next_frame().await
    }
}
    
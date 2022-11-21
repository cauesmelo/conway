use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Hello World!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

mod player;

use macroquad::prelude::*;
use player::Player;

#[macroquad::main("room game")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_text("this is the room game", 20.0, 20.0, 20.0, DARKGRAY);

        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        next_frame().await
    }
}
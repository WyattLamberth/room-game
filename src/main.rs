mod player;
mod window;
mod enemy;

use macroquad::prelude::*;
use player::Player;
use window::window_conf;
use enemy::Enemy;

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize player at screen center
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0);

    loop {
        let dt = get_frame_time();

        // Update game state
        player.update(dt);

        // Render
        
        clear_background(BLACK);
        player.draw();

        // Optional: Show FPS and position info
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(
            &format!("Position: ({:.1}, {:.1})", player.position.x, player.position.y),
            10.0,
            40.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
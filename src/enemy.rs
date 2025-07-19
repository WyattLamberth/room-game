use macroquad::prelude::*;

use crate::player;
use crate::rand;

pub struct Enemy {
    pub position: Vec2,
    pub speed: f32,
    pub radius: f32,
    color: Color,
}

impl Enemy {
    pub fn new(x:f32, y:f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            speed: 180.0, // Speed in pixels per second
            radius: 15.0, // Radius of the Enemy circle
            color: RED, // Color of the Enemy circle
        }
    }

    pub fn update(&mut self, dt: f32, player: &player::Player) {
        let direction = (player.position - self.position).normalize();

        // Apply movement with frame-rate independent speed
        self.position += direction * self.speed * dt;

        // Keep Enemy on screen
        self.clamp_to_screen();
    }

    fn clamp_to_screen(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        self.position.x = self.position.x.clamp(self.radius, screen_width - self.radius);
        self.position.y = self.position.y.clamp(self.radius, screen_height - self.radius);
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, self.radius, self.color);
        
        // Optional: draw a small dot at the center for precise positioning
        draw_circle(self.position.x, self.position.y, 2.0, WHITE);
    }
}
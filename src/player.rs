use macroquad::prelude::*;

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
    pub radius: f32,
    color: Color,
}

impl Player {
    pub fn new(x:f32, y:f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            speed: 200.0, // Speed in pixels per second
            radius: 15.0, // Radius of the player circle
            color: BLUE, // Color of the player circle
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut movement = Vec2::ZERO;

        // Handle input
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            movement.x -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            movement.x += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            movement.y -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            movement.y += 1.0;
        }

        // Normalize diagonal movement so it's not faster
        if movement.length() > 0.0 {
            movement = movement.normalize();
        }

        // Apply movement with frame-rate independent speed
        self.position += movement * self.speed * dt;

        // Keep player on screen
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
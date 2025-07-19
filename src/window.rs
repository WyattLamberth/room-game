use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Room Game".to_string(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}
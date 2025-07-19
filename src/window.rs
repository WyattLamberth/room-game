fn window_conf() -> Conf {
    pub window_title: "room game".to_string(),
    pub window_width: i32,
    pub window_height: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub sample_count: i32,
    pub window_resizable: bool,
    pub icon: Option<Icon>,
    pub platform: Platform,
}
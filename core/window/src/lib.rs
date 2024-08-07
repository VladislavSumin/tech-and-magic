use bevy::utils::default;
use bevy::window::{Window, WindowPlugin};
const WINDOW_TITLE: &str = "Tech & Magic";

pub fn create_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: WINDOW_TITLE.into(),
            ..default()
        }),
        ..default()
    }
}
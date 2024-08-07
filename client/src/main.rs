use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use core_log::create_log_plugin;
use core_window::create_window_plugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(create_window_plugin())
                .set(create_log_plugin())
        )
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default()
        ))
        .add_systems(Startup, setup)
        .run();
}



fn setup() {
    debug!("Hello, world!");
}
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy sudoku game".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    })).run();
}

use bevy::prelude::*;
use bevy_sudoku::{board::BoardPlugin, generator::GeneratePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy sudoku game".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((BoardPlugin, GeneratePlugin))
        .run();
}

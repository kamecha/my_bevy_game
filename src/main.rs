use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;

mod game;
mod game_start;
mod game_playing;
mod game_result;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "test用STGだよ~".to_string(),
                        resize_constraints: WindowResizeConstraints {
                            min_width: 1280.0,
                            min_height: 720.0,
                            max_width: 1280.0,
                            max_height: 720.0,
                        },
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
            game::GamePlugin,
        ))
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

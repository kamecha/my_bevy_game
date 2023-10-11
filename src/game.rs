use bevy::prelude::*;

use crate::game_playing::GamePlayingPlugin;
use crate::game_result::GameResultPlugin;
use crate::game_start::GameStartPlugin;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Start,
    Playing,
    Result,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            GameStartPlugin,
            GamePlayingPlugin,
            GameResultPlugin,
        ));
    }
}

use bevy::prelude::*;
use game_core::game::game_state::GameState;
use game_core::game::player::Player;
use crate::AppState;
use crate::bevy_ui::board::BoardPlugin;
use crate::bevy_ui::drag_and_drop::DragAndDropPlugin;
use crate::bevy_ui::hovering::{HighlightPlugin, ValidMovesPlugin};
use crate::bevy_ui::pieces::PiecesPlugin;

#[derive(Resource)]
pub struct GameStateRes {
    pub state: GameState,
}
impl Default for GameStateRes {
    fn default() -> Self {
        Self {
            state: GameState::new(),
        }
    }
}


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BoardPlugin,
                PiecesPlugin,
                HighlightPlugin,
                DragAndDropPlugin,
                ValidMovesPlugin,
            ))
            .insert_resource(GameStateRes {
                state: GameState::new(),
            })
            .add_systems(OnEnter(AppState::InGame), setup_game)
            .add_systems(OnExit(AppState::InGame), cleanup_game);
    }
}

fn setup_game(mut game_state: ResMut<GameStateRes>) {
    info!("Entering InGame state");
    game_state.state = GameState::new();
}

fn cleanup_game() {
    info!("Leaving InGame state");
}

#[derive(Message)]
pub struct GameOverEvent {
    pub winner: Player,
}
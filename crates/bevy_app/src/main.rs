mod bevy_ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use game_core::game::game_state::GameState;
use crate::bevy_ui::board::BoardPlugin;
use crate::bevy_ui::drag_and_drop::{GameStateRes};
use crate::bevy_ui::constants::APP_SIZE;
use crate::bevy_ui::drag_and_drop::DragAndDropPlugin;
use crate::bevy_ui::hovering::{HighlightPlugin, ValidMovesPlugin};
use crate::bevy_ui::pieces::PiecesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Noughts and Pawns".to_string(),
                    resolution: WindowResolution::new(APP_SIZE, APP_SIZE),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin{
                file_path: "../../assets".to_string(),
                ..default()
            })

        )
        .add_plugins((BoardPlugin, PiecesPlugin, HighlightPlugin, DragAndDropPlugin, ValidMovesPlugin))
        .insert_resource(GameStateRes {
            state: GameState::new(),
        })
        .run();
}

mod bevy_ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use game_core::game::game_state::GameState;
use crate::AppState::PlayMenu;
use crate::bevy_ui::board::BoardPlugin;
use crate::bevy_ui::constants::APP_SIZE;
use crate::bevy_ui::drag_and_drop::DragAndDropPlugin;
use crate::bevy_ui::hovering::{HighlightPlugin, ValidMovesPlugin};
use crate::bevy_ui::ui_main_menu::MainMenuPlugin;
use crate::bevy_ui::pieces::PiecesPlugin;
use crate::bevy_ui::ui_play_menu::PlayMenuPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    PlayMenu,
    OptionsMenu,
    InGame,
    GameOver,
}

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
        .init_state::<AppState>()
        .add_plugins((MainMenuPlugin, PlayMenuPlugin, GamePlugin))
        .run();
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

fn setup_game() {
    info!("Entering InGame state");
}

fn cleanup_game() {
    info!("Leaving InGame state");
}

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
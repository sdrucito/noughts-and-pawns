mod bevy_ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use crate::bevy_ui::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::bevy_ui::game_flow::GamePlugin;
use crate::bevy_ui::ui_game_over::GameOverPlugin;
use crate::bevy_ui::ui_main_menu::MainMenuPlugin;
use crate::bevy_ui::ui_play_menu::PlayMenuPlugin;
use crate::bevy_ui::ui_button::UIButtonPlugin;

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
        .add_plugins(EmbeddedAssetPlugin{
            mode: PluginMode::ReplaceDefault
        })
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Noughts and Pawns".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )

        .init_state::<AppState>()
        .add_plugins((MainMenuPlugin, PlayMenuPlugin, UIButtonPlugin, GamePlugin, GameOverPlugin))
        .run();
}

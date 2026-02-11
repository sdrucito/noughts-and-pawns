mod bevy_ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::bevy_ui::board::BoardPlugin;
use crate::bevy_ui::board_input::HighlightPlugin;
use crate::bevy_ui::constants::APP_SIZE;
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
        .add_plugins((BoardPlugin, PiecesPlugin, HighlightPlugin))
        .run();
}

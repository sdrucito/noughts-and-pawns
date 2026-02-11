mod bevy_ui;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use crate::bevy_ui::board::BoardPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Noughts and Pawns".to_string(),
                    resolution: WindowResolution::new(640, 640),
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
        .add_plugins(BoardPlugin)
        .run();
}

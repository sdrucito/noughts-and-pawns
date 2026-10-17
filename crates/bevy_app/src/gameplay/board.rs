use bevy::camera::ScalingMode;
use bevy::prelude::*;
use super::super::shared::constants::{WINDOW_WIDTH, WINDOW_HEIGHT, BACKGROUND_Z, BOARD_SIZE};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board);
    }
}
fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
            },
            ..OrthographicProjection::default_2d()
        }),
        Transform::default(),
        GlobalTransform::default(),
    ));

    let bg_handle = asset_server.as_ref().load("Background.png");
    //info!("Asset state: {:?}", asset_server.get_load_state(&bg_handle));

    commands.spawn((
        Sprite::from_image(bg_handle),
        Transform::from_xyz(0., 0., BACKGROUND_Z),
        GlobalTransform::default(),
    ));

    // Board
    let board_handle = asset_server.as_ref().load("sprites/board.png");

    commands.spawn((
        Sprite {
            image: board_handle,
            custom_size: Some(Vec2::splat(BOARD_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}



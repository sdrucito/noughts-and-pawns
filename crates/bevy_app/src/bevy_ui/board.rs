use bevy::camera::ScalingMode;
use bevy::prelude::*;
use crate::bevy_ui::constants::{APP_SIZE, BACKGROUND_Z, BLACK_QUAD, CELL_SIZE, GRID_NUMBER, HALF_BOARD_SIZE, WHITE_QUAD};

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
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: APP_SIZE as f32,
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
    for y in 0..GRID_NUMBER {
        for x in 0..GRID_NUMBER {
            let color = if (x + y) % 2 == 0 {
                BLACK_QUAD
            } else {
                WHITE_QUAD
            };

            let world_x = x as f32 * CELL_SIZE as f32 - HALF_BOARD_SIZE as f32 + CELL_SIZE as f32/ 2.0;
            let world_y = y as f32 * CELL_SIZE as f32 - HALF_BOARD_SIZE as f32 + CELL_SIZE as f32/ 2.0;

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(CELL_SIZE as f32)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                GlobalTransform::default(),
            ));
        }
    }
}

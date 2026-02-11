use bevy::camera::ScalingMode;
use bevy::prelude::*;

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
                viewport_height: 600.0,
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
        Transform::from_xyz(0., 0., -10.0),
        GlobalTransform::default(),
    ));

    // Board
    let cell_size = 64.0;
    let board_size = 4;
    let half = cell_size * board_size as f32 / 2.0;

    for y in 0..board_size {
        for x in 0..board_size {
            let color = if (x + y) % 2 == 0 {
                Color::srgb(0.9, 0.9, 0.9)
            } else {
                Color::srgb(0.7, 0.7, 0.7)
            };

            let world_x = x as f32 * cell_size - half + cell_size / 2.0;
            let world_y = y as f32 * cell_size - half + cell_size / 2.0;

            commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(cell_size)),
                    ..default()
                },
                Transform::from_xyz(world_x, world_y, 0.0),
                GlobalTransform::default(),
            ));
        }
    }
}

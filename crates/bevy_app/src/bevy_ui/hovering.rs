use bevy::prelude::*;
use crate::bevy_ui::constants::{HIGHLIGHT_CELL, HIGHLIGHT_Z, PIECE_Z};
use crate::bevy_ui::utils::{cell_to_world, cursor_to_world, world_to_cell};

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_highlight);
        app.add_systems(Update, update_highlight);
    }
}

#[derive(Component)]
struct CellHighlight;
fn setup_highlight(mut commands: Commands, asset_server: Res<AssetServer>) {
    let highlight_texture = asset_server.as_ref().load("cell_highlight.png");

    commands.spawn((
        Sprite{
            image:highlight_texture,
            color: HIGHLIGHT_CELL,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, HIGHLIGHT_Z),
        Visibility::Hidden,
        CellHighlight,
    ));
}
fn update_highlight(
    mut highlight_q: Query<(&mut Transform, &mut Visibility), With<CellHighlight>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((mut transform, mut visibility)) = highlight_q.single_mut() else {
        return;
    };

    if let Some(world) = cursor_to_world(&windows, &camera_q) {
        if let Some((x, y)) = world_to_cell(world) {
            transform.translation = cell_to_world(x, y).extend(PIECE_Z);
            *visibility = Visibility::Visible;
            return;
        }
    }
    *visibility = Visibility::Hidden;
}
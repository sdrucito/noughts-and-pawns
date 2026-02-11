use bevy::prelude::*;
use crate::bevy_ui::constants::{CELL_SIZE, HALF_BOARD_SIZE, PIECE_Z};
use crate::bevy_ui::pieces::{PiecesPlugin, PieceVisual};

#[derive(Component)]
struct CellHighlight;

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_highlight);
        app.init_resource::<DragState>()
            .add_systems(Update, (
                update_highlight,
                select_piece,
                drag_piece,
                drop_piece,
            ));
    }
}
fn setup_highlight(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgba(1.0, 1.0, 0.0, 0.3),
            custom_size: Some(Vec2::splat(64.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 5.0),
        GlobalTransform::default(),
        CellHighlight
    ));
}
fn update_highlight(
    mut highlight_q: Query<&mut Transform, With<CellHighlight>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if let Some(world) = cursor_to_world(&windows, &camera_q) {
        if let Some((x, y)) = world_to_cell(world) {
            if let Ok(mut transform) = highlight_q.single_mut() {
                let half = 128.0;
                let cell_size = 64.0;

                transform.translation.x = x as f32 * cell_size - half + cell_size / 2.0;
                transform.translation.y = y as f32 * cell_size - half + cell_size / 2.0;
            }
        }
    }
}

fn cursor_to_world(windows: &Query<&Window>, camera_q: &Query<(&Camera, &GlobalTransform)>) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let cursor = window.cursor_position()?;

    let (camera, camera_transform) = camera_q.single().ok()?;

    camera.viewport_to_world_2d(camera_transform, cursor).ok()
}
fn world_to_cell(world: Vec2) -> Option<(i32, i32)> {
    let x = ((world.x + HALF_BOARD_SIZE) / CELL_SIZE).floor() as i32;
    let y = ((world.y + HALF_BOARD_SIZE) / CELL_SIZE).floor() as i32;

    if x >= 0 && x < 4 && y >= 0 && y < 4 {
        Some((x, y))
    } else {
        None
    }
}
fn cell_to_world(x: i32, y: i32) -> Vec3 {
    let world_x = x as f32 * CELL_SIZE - HALF_BOARD_SIZE + CELL_SIZE / 2.0;
    let world_y = y as f32 * CELL_SIZE - HALF_BOARD_SIZE + CELL_SIZE / 2.0;

    Vec3::new(world_x, world_y, PIECE_Z)
}



#[derive(Resource, Default)]
struct DragState {
    selected: Option<Entity>,
    original_position: Option<Vec3>
}
fn select_piece(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pieces: Query<(Entity, &Transform), With<PieceVisual>>,
    mut drag_state: ResMut<DragState>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    if let Some(world) = cursor_to_world(&windows, &camera_q) {
        for (entity, transform) in pieces.iter() {
            if world.distance(transform.translation.truncate()) < 32.0 {
                drag_state.selected = Some(entity);
                drag_state.original_position = Some(transform.translation);
                break;
            }
        }
    }
}
fn drag_piece(
    drag_state: Res<DragState>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut pieces: Query<&mut Transform, With<PieceVisual>>,
) {
    if let Some(entity) = drag_state.selected {
        if let Some(world) = cursor_to_world(&windows, &camera_q) {
            if let Ok(mut transform) = pieces.get_mut(entity) {
                transform.translation.x = world.x;
                transform.translation.y = world.y;
            }
        }
    }
}
fn drop_piece(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut pieces: Query<&mut Transform, With<PieceVisual>>,
    mut drag_state: ResMut<DragState>,
) {
    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    if let Some(entity) = drag_state.selected {
        if let Ok(mut transform) = pieces.get_mut(entity) {

            if let Some(world) = cursor_to_world(&windows, &camera_q) {
                if let Some((x, y)) = world_to_cell(world) {
                    transform.translation = cell_to_world(x, y);
                } else if let Some(original) = drag_state.original_position {
                    transform.translation = original;
                }
            }
        }
    }

    drag_state.selected = None;
    drag_state.original_position = None;
}

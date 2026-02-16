use bevy::prelude::*;
use crate::bevy_ui::constants::{CELL_SIZE, HALF_BOARD_SIZE};

pub fn cursor_to_world(windows: &Query<&Window>, camera_q: &Query<(&Camera, &GlobalTransform)>) -> Option<Vec2> {
    let window = windows.single().ok()?;
    let cursor = window.cursor_position()?;

    let (camera, camera_transform) = camera_q.single().ok()?;

    camera.viewport_to_world_2d(camera_transform, cursor).ok()
}
pub fn world_to_cell(world: Vec2) -> Option<(i32, i32)> {
    let x = ((world.x + HALF_BOARD_SIZE) / CELL_SIZE).floor() as i32;
    let y = ((world.y + HALF_BOARD_SIZE) / CELL_SIZE).floor() as i32;

    if x >= 0 && x < 4 && y >= 0 && y < 4 {
        Some((x, y))
    } else {
        None
    }
}
pub fn cell_to_world(x: i32, y: i32) -> Vec2 {
    let world_x = x as f32 * CELL_SIZE - HALF_BOARD_SIZE + CELL_SIZE / 2.0;
    let world_y = y as f32 * CELL_SIZE - HALF_BOARD_SIZE + CELL_SIZE / 2.0;

    Vec2::new(world_x, world_y)
}
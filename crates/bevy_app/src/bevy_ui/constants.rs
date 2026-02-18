use bevy::color::Color;

pub const APP_SIZE: u32= 640;
pub(crate) const GRID_NUMBER: u32 = 4;

pub(crate) const CELL_SIZE: f32 = 64.0;
pub(crate) const BOARD_SIZE: f32 = CELL_SIZE*4.0;
pub(crate) const HALF_BOARD_SIZE: f32 = CELL_SIZE*2.0;

pub(crate) const WHITE_RESERVE_X: f32 = -192.0;
pub(crate) const BLACK_RESERVE_X: f32 = 192.0;
pub(crate) const BACKGROUND_Z: f32 = -10.0;
pub(crate) const PIECE_Z:f32 = 10.0;
pub(crate) const HIGHLIGHT_Z:f32 = 5.0;


pub(crate) const BLACK_QUAD: Color = Color::srgb(0.9, 0.9, 0.9);
pub(crate) const WHITE_QUAD: Color = Color::srgb(0.7, 0.7, 0.7);
pub(crate) const HIGHLIGHT_CELL: Color = Color::srgba(1.0, 1.0, 0.0, 0.5);
pub(crate) const MOVE_INDICATOR_Z: f32 = PIECE_Z - 1.0;
pub(crate) const MOVE_INDICATOR_SIZE: f32 = 28.0;
pub(crate) const CAPTURE_OVERLAY_SIZE: f32 = 64.0;
pub(crate) const MOVE_INDICATOR_FREE: Color = Color::srgba(1.0, 0.0, 0.0, 0.4);
pub(crate) const MOVE_INDICATOR_CAPTURED: Color = Color::srgba(0.0, 1.0, 0.0, 0.5);
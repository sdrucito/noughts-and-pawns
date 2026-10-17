use bevy::color::Color;

pub const APP_SIZE: u32= 640;
pub const WINDOW_WIDTH: u32 = 960;
pub const WINDOW_HEIGHT: u32 = 600;
pub const GRID_NUMBER: u32 = 4;

pub const CELL_SIZE: f32 = 64.0;
pub const BOARD_SIZE: f32 = CELL_SIZE * GRID_NUMBER as f32;

pub const HALF_BOARD_SIZE: f32 = BOARD_SIZE / 2.0;

pub const WHITE_RESERVE_X: f32 = -192.0;
pub const BLACK_RESERVE_X: f32 = 192.0;
pub const BACKGROUND_Z: f32 = -10.0;
pub const PIECE_Z:f32 = 10.0;
pub const DRAGGING_PIECE_Z: f32 = PIECE_Z + 1.0;
pub const HIGHLIGHT_Z:f32 = 5.0;


pub const BLACK_QUAD: Color = Color::srgb(0.9, 0.9, 0.9);
pub const WHITE_QUAD: Color = Color::srgb(0.7, 0.7, 0.7);
pub const HIGHLIGHT_CELL: Color = Color::srgba(1.0, 1.0, 0.0, 0.5);
pub const MOVE_INDICATOR_Z: f32 = PIECE_Z - 1.0;
pub const MOVE_INDICATOR_SIZE: f32 = 28.0;
pub const CAPTURE_OVERLAY_SIZE: f32 = 64.0;
pub const MOVE_INDICATOR_FREE: Color = Color::srgba(0.0, 1.0, 0.0, 0.4);
pub const MOVE_INDICATOR_CAPTURED: Color = Color::srgba(1.0, 0.0, 0.0, 0.5);

pub const NORMAL_BUTTON_COLOR: Color  = Color::srgb(0.3, 0.3, 0.3);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.2, 0.6, 0.9);
pub const TEXT_COLOR1: Color  = Color::srgb_u8(251, 246, 232);
pub const TEXT_COLOR2: Color  = Color::srgb_u8(43, 29, 18);


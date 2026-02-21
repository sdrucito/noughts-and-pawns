use bevy::prelude::*;
use game_core::game::game_state::Position;
use crate::bevy_ui::constants::{CAPTURE_OVERLAY_SIZE, HIGHLIGHT_CELL, HIGHLIGHT_Z, MOVE_INDICATOR_CAPTURED, MOVE_INDICATOR_FREE, MOVE_INDICATOR_SIZE, MOVE_INDICATOR_Z, PIECE_Z};
use crate::{AppState, GameStateRes};
use crate::bevy_ui::pieces::{BoardPosition, PieceVisual};
use crate::bevy_ui::utils::{cell_to_world, cursor_to_world, world_to_cell};

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_highlight);
        app.add_systems(Update, update_highlight.run_if(in_state(AppState::InGame)));
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

pub struct ValidMovesPlugin;

impl Plugin for ValidMovesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, show_valid_moves);
    }
}
#[derive(Component)]
struct MoveIndicator;
fn show_valid_moves(
    mut commands: Commands,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pieces: Query<(Entity, &PieceVisual, &BoardPosition, &Transform)>,
    indicators: Query<Entity, With<MoveIndicator>>,
    game_state: Res<GameStateRes>,
    asset_server: Res<AssetServer>,
) {
    // Cleanup
    for entity in indicators.iter() {
        commands.entity(entity).despawn();
    }

    let Some(world) = cursor_to_world(&windows, &camera_q) else { return; };
    let hovered_piece = pieces.iter().find(|(_, _, _, transform)| {
        world.distance(transform.translation.truncate()) < 32.0
    });
    let Some((_, visual, board_pos, _)) = hovered_piece else { return; };
    if visual.owner != game_state.state.current_player {
        return;
    }

    let piece_position = Position {
        x: board_pos.x as usize,
        y: board_pos.y as usize,
    };
    let moves = game_core::game::rules::valid_moves_for(&game_state.state, piece_position);
    for mv in moves {
        let pos = cell_to_world(mv.x as i32, mv.y as i32);
        let target = Position {
            x: mv.x,
            y: mv.y,
        };

        let is_capture = game_state.state.board.get(target).is_some();

        if is_capture {
            commands.spawn((
                Sprite {
                    color: MOVE_INDICATOR_FREE,
                    custom_size: Some(Vec2::splat(CAPTURE_OVERLAY_SIZE)),
                    ..default()
                },
                Transform::from_translation(pos.extend(MOVE_INDICATOR_Z)),
                MoveIndicator,
            ));
        } else {
            commands.spawn((
                Sprite {
                    image: asset_server.load("cell_highlight.png"),
                    color: MOVE_INDICATOR_CAPTURED,
                    custom_size: Some(Vec2::splat(MOVE_INDICATOR_SIZE)),
                    ..default()
                },
                Transform::from_translation(pos.extend(MOVE_INDICATOR_Z)),
                MoveIndicator,
            ));
        }
    }

}


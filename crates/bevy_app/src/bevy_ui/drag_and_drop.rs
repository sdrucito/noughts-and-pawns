use bevy::prelude::*;
use game_core::game::game_state::Position;
use game_core::game::rules::Move;
use crate::bevy_ui::constants::PIECE_Z;
use crate::bevy_ui::pieces::{reserve_position, BoardPosition, PieceVisual};
use crate::bevy_ui::utils::{cell_to_world, cursor_to_world, world_to_cell};
use crate::{AppState, GameStateRes};

pub struct DragAndDropPlugin;
impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DragState>()
            .init_resource::<GameStateRes>()
            .add_systems(Update, (
                select_piece,
                drag_piece,
                drop_piece
            ).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Resource, Default)]
struct DragState {
    selected: Option<Entity>,
    original_position: Option<Vec3>
}
impl DragState {
    pub fn rollback_position(&mut self, pieces: &mut Query<(Entity, &mut Transform, &PieceVisual, Option<&mut BoardPosition>)>) {
        if let Some(entity) = self.selected {
            if let Some(original) = self.original_position {
                if let Ok((_, mut transform, _, _)) =
                    pieces.get_mut(entity)
                {
                    transform.translation = original;
                }
            }
        }
        self.clear();
    }

    pub fn clear(&mut self) {
        self.selected = None;
        self.original_position = None;
    }
}

fn select_piece(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    pieces: Query<(Entity, &Transform, &PieceVisual)>,
    mut drag_state: ResMut<DragState>,
    game_state: Res<GameStateRes>,
) {
    // Mouse position checks
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    let Some(world) = cursor_to_world(&windows, &camera_q) else {
        return;
    };

    // Selection
    for (entity, transform, visual) in pieces.iter() {
        if world.distance(transform.translation.truncate()) < 32.0 {
            if visual.owner != game_state.state.current_player {
                warn!("Not your turn! It's {:?}'s turn.", game_state.state.current_player);
                return;
            }

            drag_state.selected = Some(entity);
            drag_state.original_position = Some(transform.translation);

            info!("{:?} selected a {:?}", visual.owner, visual.kind);

            break;
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
    mut commands: Commands,
    mut pieces: Query<(Entity, &mut Transform, &PieceVisual, Option<&mut BoardPosition>)>,
    mut drag_state: ResMut<DragState>,
    mut game_state: ResMut<GameStateRes>,
) {
    // Mouse position checks
    if !buttons.just_released(MouseButton::Left) {
        return;
    }
    let Some(entity) = drag_state.selected else {
        return;
    };
    let Some(world) = cursor_to_world(&windows, &camera_q) else {
        drag_state.rollback_position(&mut pieces);
        return;
    };
    let Some((x, y)) = world_to_cell(world) else {
        drag_state.rollback_position(&mut pieces);
        return;
    };
    let target_pos = Position { x: x as usize, y: y as usize };


    // Check if another piece already occupies the target cell (possible capture)
    let captured_entity = pieces
        .iter()
        .find(|(other_entity, _, _, other_board_pos)| {
            if *other_entity == entity {
                return false;
            }
            if let Some(pos) = other_board_pos {
                pos.x == x && pos.y == y
            } else {
                false
            }
        })
        .map(|(e, _, _, _)| e);

    // Get selected entity and release mutable borrow
    let (visual_kind, _visual_owner, old_board_pos, _original_translation) = {
        let Ok((_, transform, visual, board_pos)) = pieces.get_mut(entity)
        else {
            drag_state.clear();
            return;
        };
        (visual.kind, visual.owner, board_pos.map(|p| (p.x, p.y)), transform.translation)
    };

    // Apply move
    let result = if let Some((old_x, old_y)) = old_board_pos {
        game_core::game::rules::apply_move(
            &mut game_state.state,
            Move::MovePiece {
                from: Position {x: old_x as usize, y: old_y as usize},
                to: target_pos
            }
        )
    } else {
        game_core::game::rules::apply_move(
            &mut game_state.state,
            Move::PlacePiece {
                kind: visual_kind,
                position: target_pos
            }
        )
    };

    match result {
        Ok(_) => {
            info!("Move successful to ({}, {})", x, y);
            // Handle captured piece
            if let Some(captured) = captured_entity {
                if let Ok((_, mut captured_transform, captured_visual, _)) = pieces.get_mut(captured){
                    captured_transform.translation = reserve_position(captured_visual.owner, captured_visual.kind);
                }
                commands.entity(captured).remove::<BoardPosition>();
            }

            // Handle moved piece
            if let Ok((_, mut transform, _, _)) = pieces.get_mut(entity) {
                transform.translation = cell_to_world(x, y).extend(PIECE_Z);
            }

            commands.entity(entity).insert(BoardPosition { x, y });
        }

        Err(err) => {
            warn!("Invalid move: {}", err);
            drag_state.rollback_position(&mut pieces);
        }
    }

    drag_state.clear();
}

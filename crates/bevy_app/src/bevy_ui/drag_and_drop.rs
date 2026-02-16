use bevy::prelude::*;
use game_core::game::game_state::{GameState, Position};
use game_core::game::rules::Move;
use crate::bevy_ui::constants::PIECE_Z;
use crate::bevy_ui::pieces::{BoardPosition, PieceVisual};
use crate::bevy_ui::utils::{cell_to_world, cursor_to_world, world_to_cell};
pub struct DragAndDropPlugin;
impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DragState>()
            .init_resource::<GameStateRes>()
            .add_systems(Update, (
                select_piece,
                drag_piece,
                drop_piece,
            ));
    }
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
    pieces: Query<(Entity, &Transform, &PieceVisual)>,
    mut drag_state: ResMut<DragState>,
    game_state: Res<GameStateRes>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(world) = cursor_to_world(&windows, &camera_q) else {
        return;
    };

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
    if !buttons.just_released(MouseButton::Left) {
        return;
    }

    let Some(entity) = drag_state.selected
    else { return; };

    let Ok((entity, mut transform, visual, board_pos)) = pieces.get_mut(entity)
    else { return; };

    let Some(world) = cursor_to_world(&windows, &camera_q) else {
        restore(&mut transform, &drag_state);
        drag_state.selected = None;
        return;
    };

    if !try_drop_on_board(entity, &mut transform, visual, board_pos, world,
                          &mut commands, &mut game_state) {
        restore(&mut transform, &drag_state);
    }

    drag_state.selected = None;
}
fn restore(transform: &mut Transform, drag_state: &DragState) {
    if let Some(original) = drag_state.original_position {
        transform.translation = original;
    }
}
fn try_drop_on_board(entity: Entity, transform: &mut Transform, visual: &PieceVisual,
    board_pos: Option<Mut<BoardPosition>>, world: Vec2, commands: &mut Commands,
    game_state: &mut GameStateRes) -> bool {
    let Some((x, y)) = world_to_cell(world)
    else { return false; };

    let pos = Position { x: x as usize, y: y as usize };

    let result = if let Some(old) = board_pos {
        game_core::game::rules::apply_move(
            &mut game_state.state,
            Move::MovePiece {
                from: Position { x: old.x as usize, y: old.y as usize },
                to: pos
            })
    } else {
        game_core::game::rules::apply_move(
            &mut game_state.state,
            Move::PlacePiece {
                kind: visual.kind,
                position: pos
            })
    };

    match result {
        Ok(_) => {
            info!("Move successful to ({}, {})", x, y);
            transform.translation = cell_to_world(x, y).extend(PIECE_Z);
            commands.entity(entity).insert(BoardPosition { x, y });
            true
        }
        Err(err) => {
            warn!("Invalid move: {}", err);
            false
        }
    }
}


#[derive(Resource)]
pub struct GameStateRes {
    pub state: GameState,
}
impl Default for GameStateRes {
    fn default() -> Self {
        Self {
            state: GameState::new(),
        }
    }
}
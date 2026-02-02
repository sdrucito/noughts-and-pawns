use crate::game::piece::{Piece, PieceKind};
use crate::game::game_state::{GameState, Position};

#[derive(Debug)]
pub enum Move {
    PlacePiece {
        kind: PieceKind,
        position: Position,
    },
    MovePiece {
        from: Position,
        to: Position
    }
}

pub fn apply_move(state: &mut GameState, mv: Move) -> Result<(), String> {
    match mv {
        Move::PlacePiece { kind, position } => {
            if !state.board.is_empty_cell(position) {
                return Err("Target cell is not empty".to_string());
            }
            if !state.player_state_mut(state.current_player).has_piece(kind) {
                return Err("That piece is not available in your reserve".to_string());
            }

            let piece = Piece {
                owner: state.current_player,
                kind,
            };
            state.board.set(position, Some(piece));

            state.player_state_mut(state.current_player).remove_piece(kind);
            state.switch_turn();
            Ok(())
        }
        Move::MovePiece { from, to } => {
            validate_move_piece(state, from, to)?;

            let piece = state.board.get(from).unwrap();

            if let Some(captured) = state.board.get(to) {
                state.player_state_mut(captured.owner).add_piece(captured.kind);
            }

            state.board.set(from, None);
            state.board.set(to, Some(piece));

            state.switch_turn();
            Ok(())
        }

    }
}
fn validate_move_piece(state: &mut GameState, from: Position, to: Position) -> Result<(), String> {
    let piece = state.board.get(from)
        .ok_or("No piece at source position")?;

    if piece.owner != state.current_player {
        return Err("You can only move your own pieces".to_string());
    }

    if from == to {
        return Err("Source and destination are the same".to_string());
    }

    match piece.kind {
        PieceKind::Rook => validate_rook_move(state, from, to),
        PieceKind::Bishop => validate_bishop_move(state, from, to),
        PieceKind::Knight => validate_knight_move(state, from, to),
        PieceKind::Pawn => validate_knight_move(state, from, to)
    }
}
fn validate_rook_move(state: &mut GameState, from: Position, to: Position) -> Result<(), String> {
    let same_row = from.y == to.y;
    let same_col = from.x == to.x;

    if !same_row && !same_col {
        return Err("Rook must move in straight lines".to_string());
    }

    let dx = (to.x as isize - from.x as isize).signum();
    let dy = (to.y as isize - from.y as isize).signum();

    let mut x = from.x as isize + dx;
    let mut y = from.y as isize + dy;

    while (x as usize, y as usize) != (to.x, to.y) {
        if state.board.get(Position { x: x as usize, y: y as usize }).is_some() {
            return Err("Path is blocked".to_string());
        }
        x += dx;
        y += dy;
    }

    if let Some(dest_piece) = state.board.get(to) {
        if dest_piece.owner == state.current_player {
            return Err("Cannot capture your own piece".to_string());
        }
    }

    Ok(())
}

fn validate_bishop_move(
    state: &GameState,
    from: Position,
    to: Position,
) -> Result<(), String> {
    let dx = to.x as isize - from.x as isize;
    let dy = to.y as isize - from.y as isize;

    if dx.abs() != dy.abs() {
        return Err("Bishop must move diagonally".to_string());
    }

    let step_x = dx.signum();
    let step_y = dy.signum();

    let mut x = from.x as isize + step_x;
    let mut y = from.y as isize + step_y;

    while (x as usize, y as usize) != (to.x, to.y) {
        if state.board.get(Position { x: x as usize, y: y as usize }).is_some() {
            return Err("Path is blocked".to_string());
        }
        x += step_x;
        y += step_y;
    }

    if let Some(dest_piece) = state.board.get(to) {
        if dest_piece.owner == state.current_player {
            return Err("Cannot capture your own piece".to_string());
        }
    }

    Ok(())
}
fn validate_knight_move(
    state: &GameState,
    from: Position,
    to: Position,
) -> Result<(), String> {
    let dx = (to.x as isize - from.x as isize).abs();
    let dy = (to.y as isize - from.y as isize).abs();

    let valid = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
    if !valid {
        return Err("Knight must move in L shape".to_string());
    }

    if let Some(dest_piece) = state.board.get(to) {
        if dest_piece.owner == state.current_player {
            return Err("Cannot capture your own piece".to_string());
        }
    }

    Ok(())
}

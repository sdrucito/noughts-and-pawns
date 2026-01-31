use crate::game::piece::{Piece, PieceKind};
use crate::game::game_state::{GameState, Position};

#[derive(Debug)]
pub enum Move {
    Place {
        kind: PieceKind,
        position: Position,
    },
    // TODO: add MovePiece
}

pub fn apply_move(state: &mut GameState, mv: Move) -> Result<(), String> {
    match mv {
        Move::Place { kind, position } => {
            if !state.board.is_empty_cell(position) {
                return Err("Target cell is not empty".into());
            }
            if !state.current_player_state().has_piece(kind) {
                return Err("That piece is not available in your reserve".into());
            }

            let piece = Piece {
                owner: state.current_player,
                kind,
            };
            state.board.set(position, Some(piece));

            state.current_player_state_mut().remove_piece(kind);
            state.switch_turn();
            Ok(())
        }
    }
}

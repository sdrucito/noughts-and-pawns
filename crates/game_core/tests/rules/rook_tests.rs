use game_core::game::game_state::{GameState, Position};
use game_core::game::piece::PieceKind;
use game_core::game::player::Player;
use game_core::game::rules::{apply_move, Move};

/// Tests related to rook
mod rook_tests {
    use super::*;

    /// A rook can move horizontally across an empty row
    #[test]
    fn rook_can_move_horizontally() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Rook,
            position: Position { x: 0, y: 0 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 3, y: 0 },
        });

        // Then
        assert!(result.is_ok());
    }

    /// A rook cannot move diagonally
    #[test]
    fn rook_cannot_move_diagonally() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Rook,
            position: Position { x: 0, y: 0 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 1, y: 1 },
        });

        // Then
        assert!(result.is_err());
    }

    /// A rook can capture an opponent's piece and return it to the owner's reserve
    #[test]
    fn rook_can_capture_opponent_piece() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Rook,
            position: Position { x: 0, y: 0 },
        }).unwrap();
        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 0, y: 3 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 0, y: 3 },
        });

        // Then
        assert!(result.is_ok());
        assert!(state.board.get(Position { x: 0, y: 3 }).is_some());
        assert!(state.board.get(Position { x: 0, y: 0 }).is_none());
        assert!(state.black.has_piece(PieceKind::Pawn));
    }

    /// A rook cannot move through or onto a square blocked by another piece
    #[test]
    fn rook_cant_move_over_pieces() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Rook,
            position: Position { x: 0, y: 0 },
        }).unwrap();
        state.current_player = Player::White;
        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 0, y: 1 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result1 = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 0, y: 2 },
        });

        // Then
        assert!(result1.is_err());

        // When
        let result2 = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 0, y: 1 },
        });

        // Then
        assert!(result2.is_err());
    }
}
use noughts_and_pawns::game::game_state::{GameState, Position};
use noughts_and_pawns::game::piece::PieceKind;
use noughts_and_pawns::game::player::Player;
use noughts_and_pawns::game::rules::{apply_move, Move};

/// Tests related to bishop
mod bishop_tests {
    use super::*;

    /// A bishop can move diagonally
    #[test]
    fn bishop_can_move_diagonally() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 3, y: 3 },
        });

        // Then
        assert!(result.is_ok());
    }

    /// A bishop cannot move in a non-diagonal direction
    #[test]
    fn bishop_cannot_move_non_diagonally() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 1, y: 3 },
        });

        // Then
        assert!(result.is_err());
    }

    /// A bishop can capture an opponent's piece and return it to the owner's reserve
    #[test]
    fn bishop_can_capture() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        }).unwrap();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 3, y: 3 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 3, y: 3 },
        });

        // Then
        assert!(result.is_ok());
        assert!(state.black.has_piece(PieceKind::Pawn));
    }

    /// A bishop cannot move through another piece blocking its diagonal path
    #[test]
    fn bishop_cannot_move_through_blocking_piece() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        }).unwrap();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 3, y: 3 },
        });

        // Then
        assert!(result.is_err());
    }

    /// A bishop cannot capture a piece owned by the same player
    #[test]
    fn bishop_cannot_capture_own_piece() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        }).unwrap();

        state.current_player = Player::White;

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 3, y: 3 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 0, y: 0 },
            to: Position { x: 3, y: 3 },
        });

        // Then
        assert!(result.is_err());
    }


}
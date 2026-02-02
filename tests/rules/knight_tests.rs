use noughts_and_pawns::game::game_state::{GameState, Position};
use noughts_and_pawns::game::piece::PieceKind;
use noughts_and_pawns::game::player::Player;
use noughts_and_pawns::game::rules::{apply_move, Move};

/// Tests related to knight
mod knight_tests {
    use super::*;

    /// A knight can move in L-shaped pattern
    #[test]
    fn knight_can_jump_over_pieces() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Knight,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 2 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 2, y: 3 },
        });

        // Then
        assert!(result.is_ok());
    }

    /// A knight cannot move in a non-L-shaped pattern
    #[test]
    fn knight_cannot_move_non_l_shape() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Knight,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 2, y: 2 },
        });

        // Then
        assert!(result.is_err());
    }

    ///A knight can capture an opponent's piece and return it to the owner's reserve
    #[test]
    fn knight_can_capture() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Knight,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 2, y: 3 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 2, y: 3 },
        });

        // Then
        assert!(result.is_ok());
        assert!(state.black.has_piece(PieceKind::Pawn));
    }

    /// A knight cannot capture a piece owned by the same player
    #[test]
    fn knight_cannot_capture_own_piece() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Knight,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 2, y: 3 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 2, y: 3 },
        });

        // Then
        assert!(result.is_err());
    }

}
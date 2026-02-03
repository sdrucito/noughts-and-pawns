use noughts_and_pawns::game::game_state::{GameState, Position};
use noughts_and_pawns::game::piece::PieceKind;
use noughts_and_pawns::game::player::Player;
use noughts_and_pawns::game::rules::{apply_move, Move};

mod pawn_tests {
    use super::*;

    /// A pawn can move one square forward into an empty cell
    #[test]
    fn pawn_can_move_forward() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 0 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 0 },
            to: Position { x: 1, y: 1 },
        });

        // Then
        assert!(result.is_ok());
    }

    /// A pawn cannot move forward into an occupied square
    #[test]
    fn pawn_cannot_move_forward_into_occupied_cell() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 0 },
        }).unwrap();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 1 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 0 },
            to: Position { x: 1, y: 1 },
        },
        );

        // Then
        assert!(result.is_err());
    }

    /// A pawn can capture an opponent piece diagonally
    #[test]
    fn pawn_can_capture_diagonally() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 2, y: 2 },
        }).unwrap();
        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 2, y: 2 },
        });

        // Then
        assert!(result.is_ok());
        assert!(state.black.has_piece(PieceKind::Pawn));
    }

    /// A pawn cannot move diagonally without capturing a piece
    #[test]
    fn pawn_cannot_move_diagonally_without_capture() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
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

    /// A pawn cannot capture a piece owned by the same player
    #[test]
    fn pawn_cannot_capture_own_piece() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Rook,
            position: Position { x: 2, y: 2 },
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

    /// A pawn reverses its movement direction when reaching the edge of the board
    #[test]
    fn pawn_reverses_direction_at_board_edge() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 2 },
        }).unwrap();
        state.current_player = Player::White;

        apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 2 },
            to: Position { x: 1, y: 3 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 3 },
            to: Position { x: 1, y: 2 },
        });

        // Then
        assert!(result.is_ok());
    }

    /// A pawn cannot move backward before its direction has been reversed
    #[test]
    fn pawn_cannot_move_backward_before_reaching_edge() {
        // Given
        let mut state = GameState::new();

        apply_move(&mut state, Move::PlacePiece {
            kind: PieceKind::Pawn,
            position: Position { x: 1, y: 1 },
        }).unwrap();

        state.current_player = Player::White;

        // When
        let result = apply_move(&mut state, Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 1, y: 0 },
        });

        // Then
        assert!(result.is_err());
    }
}

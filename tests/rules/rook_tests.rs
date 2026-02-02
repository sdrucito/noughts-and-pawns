use noughts_and_pawns::game::game_state::{GameState, Position};
use noughts_and_pawns::game::piece::PieceKind;
use noughts_and_pawns::game::player::Player;
use noughts_and_pawns::game::rules::{apply_move, Move};

mod rook_tests {
    use super::*;
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
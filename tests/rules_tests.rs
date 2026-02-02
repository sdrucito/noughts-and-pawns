use noughts_and_pawns::game::game_state::{GameState, Position};
use noughts_and_pawns::game::piece::PieceKind;
use noughts_and_pawns::game::player::Player;
use noughts_and_pawns::game::rules::{apply_move, Move};
mod rules;

/// Tests related to generic move validation rules
mod rules_tests {
    use super::*;

    /// A piece cannot be placed on an already occupied cell
    #[test]
    fn cannot_place_piece_on_occupied_cell() {
        // Given
        let mut state = GameState::new();

        let mv1 = Move::PlacePiece {
            kind: PieceKind::Rook,
            position: Position { x: 1, y: 1 },
        };

        let mv2 = Move::PlacePiece {
            kind: PieceKind::Knight,
            position: Position { x: 1, y: 1 },
        };

        // When
        apply_move(&mut state, mv1).unwrap();
        let result = apply_move(&mut state, mv2);

        // Then
        assert!(result.is_err());
    }

    /// A player cannot place the same piece twice from their reserve
    #[test]
    fn cannot_place_same_piece_twice() {
        // Given
        let mut state = GameState::new();

        let white_move1 = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        };

        let black_move_err = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        };
        let black_move_right = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 1, y: 1 },
        };
        let white_move2 = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 2, y: 2 },
        };

        // Then
        assert!(apply_move(&mut state, white_move1).is_ok());
        assert!(apply_move(&mut state, black_move_err).is_err());
        apply_move(&mut state, black_move_right).unwrap();

        let result = apply_move(&mut state, white_move2);
        assert!(result.is_err());
    }

    /// A player cannot move a piece of another player
    #[test]
    fn cannot_move_piece_of_another_player() {
        // Given
        let mut state = GameState::new();

        let white_move1 = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        };
        let black_move = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 1, y: 1 },
        };
        let white_move2 = Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 2, y: 2 },
        };

        // When
        apply_move(&mut state, white_move1).unwrap();
        apply_move(&mut state, black_move).unwrap();
        let result = apply_move(&mut state, white_move2);

        // Then
        assert!(result.is_err());
    }

    /// A piace cannot move on itself
    #[test]
    fn cannot_move_piece_in_self_position() {
        // Given
        let mut state = GameState::new();

        let white_move1 = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 0, y: 0 },
        };
        let black_move = Move::PlacePiece {
            kind: PieceKind::Bishop,
            position: Position { x: 1, y: 1 },
        };
        let white_move2 = Move::MovePiece {
            from: Position { x: 1, y: 1 },
            to: Position { x: 1, y: 1 },
        };

        // When
        apply_move(&mut state, white_move1).unwrap();
        apply_move(&mut state, black_move).unwrap();
        let result = apply_move(&mut state, white_move2);

        // Then
        assert!(result.is_err());
    }

}
mod win_condition_tests {
    use noughts_and_pawns::game::game_state::{GameState, Position};
    use noughts_and_pawns::game::piece::{Piece, PieceKind};
    use noughts_and_pawns::game::player::Player;
    use noughts_and_pawns::game::rules::check_win_condition;
    use super::*;

    /// A player wins with four pieces aligned horizontally
    #[test]
    fn win_with_horizontal_line() {
        // Given
        let mut state = GameState::new();

        // When
        for x in 0..4 {
            state.board.set(
                Position { x, y: 0 },
                Some(Piece {
                    owner: Player::White,
                    kind: PieceKind::Pawn,
                    pawn_dir: None,
                }),
            );
        }

        // Then
        assert!(check_win_condition(&state, Player::White));
    }

    /// A player wins with four pieces aligned vertically
    #[test]
    fn win_with_vertical_line() {
        // Given
        let mut state = GameState::new();

        // When
        for y in 0..4 {
            state.board.set(
                Position { x: 1, y },
                Some(Piece {
                    owner: Player::White,
                    kind: PieceKind::Pawn,
                    pawn_dir: None,
                }),
            );
        }

        // Then
        assert!(check_win_condition(&state, Player::White));
    }

    /// A player wins with four pieces on the main diagonal
    #[test]
    fn win_with_main_diagonal() {
        // Given
        let mut state = GameState::new();

        // When
        for i in 0..4 {
            state.board.set(
                Position { x: i, y: i },
                Some(Piece {
                    owner: Player::White,
                    kind: PieceKind::Pawn,
                    pawn_dir: None,
                }),
            );
        }

        // Then
        assert!(check_win_condition(&state, Player::White));
    }

    /// A player wins with four pieces on the secondary diagonal
    #[test]
    fn win_with_secondary_diagonal() {
        // Given
        let mut state = GameState::new();

        // When
        for i in 0..4 {
            state.board.set(
                Position { x: 3 - i, y: i },
                Some(Piece {
                    owner: Player::White,
                    kind: PieceKind::Pawn,
                    pawn_dir: None,
                }),
            );
        }

        // Test
        assert!(check_win_condition(&state, Player::White));
    }

    /// A line with mixed player pieces does not trigger a win
    #[test]
    fn no_win_with_mixed_player_line() {
        let mut state = GameState::new();

        state.board.set(Position { x: 0, y: 0 }, Some(Piece { owner: Player::White, kind: PieceKind::Pawn, pawn_dir: None }));
        state.board.set(Position { x: 1, y: 0 }, Some(Piece { owner: Player::Black, kind: PieceKind::Pawn, pawn_dir: None }));
        state.board.set(Position { x: 2, y: 0 }, Some(Piece { owner: Player::White, kind: PieceKind::Rook, pawn_dir: None }));
        state.board.set(Position { x: 3, y: 0 }, Some(Piece { owner: Player::White, kind: PieceKind::Bishop, pawn_dir: None }));

        assert!(!check_win_condition(&state, Player::White));
    }

}
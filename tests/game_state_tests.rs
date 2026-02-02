use noughts_and_pawns::game::game_state::{GameState, Position};
use noughts_and_pawns::game::piece::PieceKind;
use noughts_and_pawns::game::player::Player;

/// Tests related to GameState
mod game_state_tests {
    use super::*;

    /// The game starts with an empty board
    #[test]
    fn game_starts_with_empty_board() {
        // Given
        let state = GameState::new();

        // Then
        for y in 0..4 {
            for x in 0..4 {
                assert!(state.board.get(Position { x, y }).is_none());
            }
        }
    }

    /// The game starts with White as the current player
    #[test]
    fn game_starts_with_white_player() {
        // Given
        let state = GameState::new();

        // Then
        assert_eq!(state.current_player, Player::White);
    }

    /// Switching turn alternates the current player
    #[test]
    fn switch_turn_alternates_player() {
        // Given
        let mut state = GameState::new();

        // When
        state.switch_turn();

        // Then
        assert_eq!(state.current_player, Player::Black);

        // When
        state.switch_turn();

        // Then
        assert_eq!(state.current_player, Player::White);
    }

    /// current_player_state_mut allows mutating the active player's state
    #[test]
    fn current_player_state_mut_returns_mutable_state() {
        // Given
        let mut state = GameState::new();

        // When
        state.player_state_mut(state.current_player).remove_piece(PieceKind::Pawn);

        // Then
        assert!(!state.player_state(state.current_player).has_piece(PieceKind::Pawn));
    }

    /// player_state_mut returns the correct PlayerState for a given player
    #[test]
    fn player_state_returns_correct_state_for_both_players() {
        // Given
        let mut state = GameState::new();

        // When
        state.player_state_mut(Player::Black).remove_piece(PieceKind::Pawn);

        // Then
        assert!(!state.player_state(Player::Black).has_piece(PieceKind::Pawn));
        assert!(state.player_state(Player::White).has_piece(PieceKind::Pawn));
    }
}
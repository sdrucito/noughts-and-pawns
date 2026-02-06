use game_core::game::piece::PieceKind;
use game_core::game::player::{Player, PlayerState, Reserve};

/// Tests related to Player
mod player_tests {
    use super::*;

    /// The reserve starts with all pieces
    #[test]
    fn reserve_starts_with_all_pieces() {
        // Given
        let reserve = Reserve::new();

        // Then
        assert!(reserve.has_piece(PieceKind::Pawn));
        assert!(reserve.has_piece(PieceKind::Rook));
        assert!(reserve.has_piece(PieceKind::Knight));
        assert!(reserve.has_piece(PieceKind::Bishop));
    }

    /// Add and remove all pieces
    #[test]
    fn reserve_remove_and_add_piece() {
        // Given
        let mut reserve = Reserve::new();
        let pieces = [PieceKind::Pawn,
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
        ];

        // Then
        for kind in pieces {
            reserve.remove_piece(kind);
            assert!(!reserve.has_piece(kind));

            reserve.add_piece(kind);
            assert!(reserve.has_piece(kind));
        }
    }

    ///Add and remove update PlayerState
    #[test]
    fn player_state_remove_and_add_piece() {
        // Given
        let mut black = PlayerState::new(Player::Black);

        // When
        black.remove_piece(PieceKind::Knight);
        // Then
        assert!(!black.has_piece(PieceKind::Knight));

        // When
        black.add_piece(PieceKind::Knight);
        // Then
        assert!(black.has_piece(PieceKind::Knight));
    }

    /// Check color of a new PlayerState
    #[test]
    fn get_player_color_from_player_state(){
        // Given
        let black = PlayerState::new(Player::Black);
        let white = PlayerState::new(Player::White);

        // Then
        assert_eq!(black.get_color(),Player::Black);
        assert_eq!(white.get_color(),Player::White);

    }
}
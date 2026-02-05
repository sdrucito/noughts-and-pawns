use crate::game::game_state::{GameState, Position, BOARD_SIZE};
use crate::game::piece::PieceKind;
use crate::game::player::Player;

/// Prints the game instructions at startup
pub fn print_instructions() {
    println!("Noughts and Pawns\n");
    println!("White pieces are shown in UPPERCASE");
    println!("Black pieces are shown in lowercase\n");

    println!("Goal:");
    println!("Create a line of 4 of your pieces horizontally, vertically or diagonally.\n");

    println!("Commands:");
    println!("  place <piece> <position>   (e.g. place pawn B2)");
    println!("  move <from> <to>           (e.g. move B2 B3)\n");

    println!("Pieces:");
    println!("  pawn (P/p), rook (R/r), knight (K/k), bishop (B/b)\n");
}

/// Prints the board with coordinates and player reserves
pub fn print_game_state(state: &GameState) {
    print_column_headers();
    print_board_rows(state);
    print_column_headers();

    println!("\nCurrent player: {:?}\n", state.current_player);
}

fn print_column_headers() {
    print!("    |");
    for x in 0..BOARD_SIZE {
        let col = (b'A' + x as u8) as char;
        print!("  {}", col);
    }
    println!("  |");
}

fn print_board_rows(state: &GameState) {
    for y in (0..BOARD_SIZE).rev() {
        // Left reserve (White)
        print!(
            "{} | {} ",
            reserve_symbol(&state.white, row_piece_kind(y), true),
            y + 1
        );

        // Board cells
        for x in 0..BOARD_SIZE {
            let symbol = match state.board.get(Position { x, y }) {
                Some(piece) => piece_char(piece.owner, piece.kind),
                None => '.',
            };
            print!(" {} ", symbol);
        }

        // Right side row index + Black reserve
        println!(
            " {} | {}",
            y + 1,
            reserve_symbol(&state.black, row_piece_kind(y), false)
        );
    }
}

/// Maps a row index to a piece kind for reserve display
fn row_piece_kind(row: usize) -> PieceKind {
    match row {
        3 => PieceKind::Pawn,
        2 => PieceKind::Rook,
        1 => PieceKind::Knight,
        _ => PieceKind::Bishop,
    }
}

/// Returns the character for a piece on the board
fn piece_char(owner: Player, kind: PieceKind) -> char {
    let c = match kind {
        PieceKind::Pawn => 'P',
        PieceKind::Rook => 'R',
        PieceKind::Knight => 'K',
        PieceKind::Bishop => 'B',
    };

    match owner {
        Player::White => c,
        Player::Black => c.to_ascii_lowercase(),
    }
}

/// Returns the symbol for a piece in the reserve (letter if available, '.' if not)
fn reserve_symbol(
    player_state: &crate::game::player::PlayerState,
    kind: PieceKind,
    white: bool,
) -> char {
    if player_state.has_piece(kind) {
        let c = match kind {
            PieceKind::Pawn => 'P',
            PieceKind::Rook => 'R',
            PieceKind::Knight => 'K',
            PieceKind::Bishop => 'B',
        };
        if white { c } else { c.to_ascii_lowercase() }
    } else {
        '.'
    }
}

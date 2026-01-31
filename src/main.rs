mod game;

use std::io;
use std::io::Write;
use game::piece::PieceKind;
use game::game_state::{GameState, Position, BOARD_SIZE};
use game::rules::apply_move;
use game::rules::Move;
use crate::game::player::Player;

fn print_board(state: &GameState) {
    println!("Current player: {:?}\n", state.current_player);

    print!("  ");
    for x in 1..=BOARD_SIZE {
        print!(" {} ", x);
    }
    println!();
    for y in 0..BOARD_SIZE {
        print!("{} ", y + 1);
        for x in 0..BOARD_SIZE {
            match state.board.get(Position { x, y }) {
                Some(piece) => {
                    let c = match piece.kind {
                        PieceKind::Pawn => 'P',
                        PieceKind::Rook => 'R',
                        PieceKind::Knight => 'N',
                        PieceKind::Bishop => 'B',
                    };
                    let owner = match piece.owner {
                        Player::White => c,
                        Player::Black => c.to_ascii_lowercase(),
                    };
                    print!(" {} ", owner);
                }
                None => print!(" . "),
            }
        }
        println!();
    }
    println!();
}
fn parse_piece_kind(s: &str) -> Option<PieceKind> {
    match s.to_lowercase().as_str() {
        "pawn" => Some(PieceKind::Pawn),
        "rook" => Some(PieceKind::Rook),
        "knight" => Some(PieceKind::Knight),
        "bishop" => Some(PieceKind::Bishop),
        _ => None,
    }
}

fn parse_command(input: &str) -> Result<Move, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 4 {
        return Err("Invalid command format. Use: place <piece> <x> <y>".into());
    }

    let kind = parse_piece_kind(parts[1])
        .ok_or("Unknown piece type")?;

    let x: usize = parts[2].parse().map_err(|_| "Invalid x coordinate")?;
    let y: usize = parts[3].parse().map_err(|_| "Invalid y coordinate")?;

    if x == 0 || y == 0 || x > BOARD_SIZE || y > BOARD_SIZE {
        return Err("Coordinates out of board range".into());
    }

    Ok(Move::Place {
        kind,
        position: Position {
            x: x - 1,
            y: y - 1,
        },
    })
}

fn read_input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
fn print_help() {
    println!("Commands:");
    println!("  place <piece> <x> <y>");
    println!("Pieces: pawn, rook, knight, bishop");
    println!("Coordinates: 1 to 4");
}
fn main() {
    println!("Start new game \n");
    print_help();
    let mut state = GameState::new();

    loop {
        print_board(&state);

        println!("Player {:?}, enter your move:", state.current_player);

        let input = read_input();

        let mv = match parse_command(&input) {
            Ok(mv) => mv,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        if let Err(e) = apply_move(&mut state, mv) {
            println!("Illegal move: {}", e);
            continue;
        }
    }

}


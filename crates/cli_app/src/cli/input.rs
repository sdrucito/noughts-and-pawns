use game_core::game::game_state::{BOARD_SIZE, Position};
use game_core::game::piece::PieceKind;
use game_core::game::rules::Move;

/// Parses a user command into a Move
pub fn parse_command(input: &String) -> Result<Move, String> {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    match parts[0].to_lowercase().as_str() {
        "place" => parse_place_command(&parts),
        "move" => parse_move_command(&parts),
        _ => Err("Unknown command. Use 'place' or 'move'.".to_string()),
    }
}

fn parse_place_command(parts: &[&str]) -> Result<Move, String> {
    if parts.len() != 3 {
        return Err("Usage: place <piece> <position>".to_string());
    }

    let kind = parse_piece_kind(parts[1])?;
    let position = parse_position(parts[2])?;

    Ok(Move::PlacePiece { kind, position })
}

fn parse_move_command(parts: &[&str]) -> Result<Move, String> {
    if parts.len() != 3 {
        return Err("Usage: move <from> <to>".to_string());
    }

    let from = parse_position(parts[1])?;
    let to = parse_position(parts[2])?;

    Ok(Move::MovePiece { from, to })
}

/// Parses a position like "B3" or "a1"
fn parse_position(s: &str) -> Result<Position, String> {
    let chars: Vec<char> = s.chars().collect();

    if chars.len() != 2 {
        return Err("Position must be in the form <Letter><Number> (e.g. B2)".to_string());
    }

    let col = chars[0].to_ascii_uppercase();
    let row = chars[1];

    if !('A'..='Z').contains(&col) {
        return Err("Invalid column letter".to_string());
    }

    if !('1'..='9').contains(&row) {
        return Err("Invalid row number".to_string());
    }

    let x = (col as u8 - b'A') as usize;
    let y = (row as u8 - b'1') as usize;

    if x >= BOARD_SIZE || y >= BOARD_SIZE {
        return Err("Position out of board range".to_string());
    }

    Ok(Position { x, y })
}

fn parse_piece_kind(s: &str) -> Result<PieceKind, String> {
    match s.to_lowercase().as_str() {
        "pawn" => Ok(PieceKind::Pawn),
        "rook" => Ok(PieceKind::Rook),
        "bishop" => Ok(PieceKind::Bishop),
        "knight" => Ok(PieceKind::Knight),
        _ => Err("Unknown piece. Use pawn, rook, bishop or knight.".to_string()),
    }
}

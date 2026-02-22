use crate::game::piece::{PawnDirection, Piece, PieceKind};
use crate::game::game_state::{BOARD_SIZE, GameState, Position};
use crate::game::player::Player;

#[derive(Debug)]
pub enum Move {
    PlacePiece {
        kind: PieceKind,
        position: Position,
    },
    MovePiece {
        from: Position,
        to: Position
    }
}

pub fn apply_move(state: &mut GameState, mv: Move) -> Result<(), String> {
    match mv {
        Move::PlacePiece { kind, position } => {
            if !state.board.is_empty_cell(position) {
                return Err("Target cell is not empty".to_string());
            }
            if !state.player_state_mut(state.current_player).has_piece(kind) {
                return Err("That piece is not available in your reserve".to_string());
            }

            let pawn_dir = if kind == PieceKind::Pawn {
                Some(PawnDirection::Forward)
            } else {
                None
            };

            let piece = Piece {
                owner: state.current_player,
                kind,
                pawn_dir,
            };

            state.board.set(position, Some(piece));

            state.player_state_mut(state.current_player).remove_piece(kind);

            if let Some(winner) = check_win_condition(state, state.current_player) {
                state.winner = Some(winner);
            }

            state.switch_turn();
            Ok(())
        }
        Move::MovePiece { from, to } => {
            validate_move_piece(state, from, to)?;

            let mut piece = state.board.get(from).unwrap();

            if let Some(captured) = state.board.get(to) {
                state.player_state_mut(captured.owner).add_piece(captured.kind);
            }

            if piece.kind == PieceKind::Pawn {
                let forward = pawn_forward_delta(&piece);
                let next_y = to.y as isize + forward;

                let reached_edge =
                    next_y < 0 || next_y >= BOARD_SIZE as isize;

                if reached_edge {
                    piece.pawn_dir = Some(match piece.pawn_dir.unwrap() {
                        PawnDirection::Forward => PawnDirection::Backward,
                        PawnDirection::Backward => PawnDirection::Forward,
                    });
                }
            }

            state.board.set(from, None);
            state.board.set(to, Some(piece));

            if let Some(winner) = check_win_condition(state, state.current_player) {
                state.winner = Some(winner);
            }

            state.switch_turn();
            Ok(())
        }
    }
}
fn validate_move_piece(state: &GameState, from: Position, to: Position) -> Result<(), String> {
    let piece = state.board.get(from).ok_or("No piece at source position")?;

    if piece.owner != state.current_player {
        return Err("You can only move your own pieces".to_string());
    }

    if from == to {
        return Err("Source and destination are the same".to_string());
    }

    match piece.kind {
        PieceKind::Rook => validate_rook_move(state, from, to),
        PieceKind::Bishop => validate_bishop_move(state, from, to),
        PieceKind::Knight => validate_knight_move(state, from, to),
        PieceKind::Pawn => validate_pawn_move(state, from, to)
    }
}
fn validate_rook_move(state: &GameState, from: Position, to: Position) -> Result<(), String> {
    let same_row = from.y == to.y;
    let same_col = from.x == to.x;

    if !same_row && !same_col {
        return Err("Rook must move in straight lines".to_string());
    }

    let dx = (to.x as isize - from.x as isize).signum();
    let dy = (to.y as isize - from.y as isize).signum();

    let mut x = from.x as isize + dx;
    let mut y = from.y as isize + dy;

    while (x as usize, y as usize) != (to.x, to.y) {
        if state.board.get(Position { x: x as usize, y: y as usize }).is_some() {
            return Err("Path is blocked".to_string());
        }
        x += dx;
        y += dy;
    }

    if let Some(dest_piece) = state.board.get(to) {
        if dest_piece.owner == state.current_player {
            return Err("Cannot capture your own piece".to_string());
        }
    }

    Ok(())
}

fn validate_bishop_move(state: &GameState, from: Position, to: Position) -> Result<(), String> {
    let dx = to.x as isize - from.x as isize;
    let dy = to.y as isize - from.y as isize;

    if dx.abs() != dy.abs() {
        return Err("Bishop must move diagonally".to_string());
    }

    let step_x = dx.signum();
    let step_y = dy.signum();

    let mut x = from.x as isize + step_x;
    let mut y = from.y as isize + step_y;

    while (x as usize, y as usize) != (to.x, to.y) {
        if state.board.get(Position { x: x as usize, y: y as usize }).is_some() {
            return Err("Path is blocked".to_string());
        }
        x += step_x;
        y += step_y;
    }

    if let Some(dest_piece) = state.board.get(to) {
        if dest_piece.owner == state.current_player {
            return Err("Cannot capture your own piece".to_string());
        }
    }

    Ok(())
}
fn validate_knight_move(state: &GameState, from: Position, to: Position) -> Result<(), String> {
    let dx = (to.x as isize - from.x as isize).abs();
    let dy = (to.y as isize - from.y as isize).abs();

    let valid = (dx == 2 && dy == 1) || (dx == 1 && dy == 2);
    if !valid {
        return Err("Knight must move in L shape".to_string());
    }

    if let Some(dest_piece) = state.board.get(to) {
        if dest_piece.owner == state.current_player {
            return Err("Cannot capture your own piece".to_string());
        }
    }

    Ok(())
}

fn validate_pawn_move(state: &GameState, from: Position, to: Position) -> Result<(), String> {
    let piece = state.board.get(from).unwrap();

    let dx = to.x as isize - from.x as isize;
    let dy = to.y as isize - from.y as isize;

    let forward = pawn_forward_delta(&piece);

    if dx == 0 && dy == forward {
        if state.board.get(to).is_some() {
            return Err("Pawn cannot move forward into an occupied square".to_string());
        }
        return Ok(());
    }

    if dy == forward && dx.abs() == 1 {
        return if let Some(dest_piece) = state.board.get(to) {
            if dest_piece.owner == piece.owner {
                return Err("Cannot capture your own piece".to_string());
            }
            Ok(())
        } else {
            Err("Pawn can only move diagonally when capturing".to_string())
        };
    }

    Err("Invalid pawn move".to_string())
}

fn pawn_forward_delta(piece: &Piece) -> isize {
    match (piece.owner, piece.pawn_dir.unwrap()) {
        (Player::White, PawnDirection::Forward) => 1,
        (Player::White, PawnDirection::Backward) => -1,
        (Player::Black, PawnDirection::Forward) => -1,
        (Player::Black, PawnDirection::Backward) => 1,
    }
}

pub fn check_win_condition(state: &GameState, player: Player) -> Option<Player> {
    // Rows
    for y in 0..BOARD_SIZE {
        if (0..BOARD_SIZE).all(|x| cell_belongs_to_player(state, x, y, player)) {
            return Some(player);
        }
    }
    // Columns
    for x in 0..BOARD_SIZE {
        if (0..BOARD_SIZE).all(|y| cell_belongs_to_player(state, x, y, player)) {
            return Some(player);
        }
    }
    // Diagonals
    if (0..BOARD_SIZE).all(|i| cell_belongs_to_player(state, i, i, player)) {
        return Some(player);
    }
    if (0..BOARD_SIZE).all(|i| cell_belongs_to_player(state, BOARD_SIZE - 1 - i, i, player)) {
        return Some(player);
    }

    None
}

fn cell_belongs_to_player(state: &GameState, x: usize, y: usize, player: Player) -> bool {
    state.board.get(Position { x, y })
        .map(|p| p.owner == player).unwrap_or(false)
}

pub fn valid_moves_for(state: &GameState, position: Position) -> Vec<Position> {
    let Some(piece) = state.board.get(position) else {
        return vec![];
    };
    if piece.owner != state.current_player {
        return vec![];
    }

    let mut moves = Vec::new();

    for y in 0..BOARD_SIZE {
        for x in 0..BOARD_SIZE {
            let to = Position { x, y };

            if position == to {
                continue;
            }

            if validate_move_piece(state, position, to).is_ok() {
                moves.push(to);
            }
        }
    }

    moves
}

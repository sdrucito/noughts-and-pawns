use crate::game::piece::Piece;
use crate::game::player::{Player, PlayerState};

pub const BOARD_SIZE: usize = 4;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Board {
    cells: [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE],
}
impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn get(&self, pos: Position) -> Option<Piece> {
        self.cells[pos.y][pos.x]
    }

    pub fn set(&mut self, pos: Position, piece: Option<Piece>) {
        self.cells[pos.y][pos.x] = piece;
    }

    pub fn is_empty_cell(&self, pos: Position) -> bool {
        self.get(pos).is_none()
    }
}

pub struct GameState {
    pub board: Board,
    pub current_player: Player,
    pub white: PlayerState,
    pub black: PlayerState
}
impl GameState {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::White,
            white: PlayerState::new(Player::White),
            black: PlayerState::new(Player::Black)
        }
    }
    pub fn player_state(&self, player: Player) -> &PlayerState {
        match player {
            Player::White => &self.white,
            Player::Black => &self.black,
        }
    }
    pub fn player_state_mut(&mut self, player: Player) -> &mut PlayerState {
        match player {
            Player::White => &mut self.white,
            Player::Black => &mut self.black,
        }
    }
    pub fn switch_turn(&mut self) {
        self.current_player = self.current_player.opposite();
    }
}
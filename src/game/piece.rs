use crate::game::player::Player;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub owner: Player,
    pub kind: PieceKind,
}


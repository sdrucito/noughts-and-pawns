use crate::game::player::Player;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub owner: Player,
    pub kind: PieceKind,
}


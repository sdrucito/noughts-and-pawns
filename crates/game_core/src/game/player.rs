use crate::game::piece::PieceKind;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    White,
    Black,
}
impl Player {
    pub fn opposite(self) -> Player { //TODO: check if necessary
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

pub struct Reserve {
    pawn: bool,
    rook: bool,
    knight: bool,
    bishop: bool
}
impl Reserve {
    pub fn new()->Self{
        Self{
            pawn: true,
            rook: true,
            knight: true,
            bishop: true,
        }
    }

    pub fn has_piece(&self, kind: PieceKind) -> bool {
        return match kind {
            PieceKind::Pawn => self.pawn,
            PieceKind::Rook => self.rook,
            PieceKind::Knight=> self.knight,
            PieceKind::Bishop=> self.bishop,
        }
    }
    pub fn add_piece(&mut self, kind: PieceKind){
        match kind {
            PieceKind::Pawn => self.pawn=true,
            PieceKind::Rook => self.rook=true,
            PieceKind::Knight=> self.knight=true,
            PieceKind::Bishop=> self.bishop=true
        }
    }
    pub fn remove_piece(&mut self, kind: PieceKind){
        match kind {
            PieceKind::Pawn => self.pawn=false,
            PieceKind::Rook => self.rook=false,
            PieceKind::Knight=> self.knight=false,
            PieceKind::Bishop=> self.bishop=false
        }
    }
}

pub struct PlayerState{
    color: Player,
    reserve: Reserve
}
impl PlayerState{
    pub fn new(p: Player)->Self{
        Self{
            color:p,
            reserve: Reserve::new()
        }
    }
    pub fn has_piece(&self, kind: PieceKind) -> bool {
        self.reserve.has_piece(kind)
    }

    pub fn remove_piece(&mut self, kind: PieceKind) {
        self.reserve.remove_piece(kind);
    }

    pub fn add_piece(&mut self, kind: PieceKind) {
        self.reserve.add_piece(kind);
    }

    pub fn get_color(&self) -> Player {
        self.color
    }
}
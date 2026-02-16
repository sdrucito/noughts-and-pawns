use bevy::prelude::*;
use game_core::game::piece::PieceKind;
use game_core::game::player::Player;
use crate::bevy_ui::constants::{BLACK_RESERVE_X, WHITE_RESERVE_X, PIECE_Z};
const RESERVE_LAYOUT: [ReserveSlot; 4] = [
    ReserveSlot { kind: PieceKind::Pawn,   y: 108.0 },
    ReserveSlot { kind: PieceKind::Rook,   y: 36.0 },
    ReserveSlot { kind: PieceKind::Knight, y: -36.0 },
    ReserveSlot { kind: PieceKind::Bishop, y: -108.0 },
];
struct ReserveSlot {
    kind: PieceKind,
    y: f32,
}
#[derive(Component)]
pub struct BoardPosition {
    pub x: i32,
    pub y: i32,
}
#[derive(Component)]
pub struct PieceVisual {
    pub owner: Player,
    pub kind: PieceKind,
}
pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_reserve_pieces);
    }
}

fn spawn_reserve_pieces(mut commands: Commands, asset_server: Res<AssetServer>) {
    for piece in RESERVE_LAYOUT {
        spawn_piece_in_reserve(&mut commands, asset_server.as_ref(), Player::White, piece.kind);
    }
    for piece in RESERVE_LAYOUT {
        spawn_piece_in_reserve(&mut commands, asset_server.as_ref(), Player::Black, piece.kind);
    }
}

fn spawn_piece(commands: &mut Commands, asset_server: &AssetServer, owner: Player,
                   kind: PieceKind, position: Vec3) -> Entity {
    let texture = asset_server.load(sprite_path(owner, kind));
    commands
        .spawn((
            Sprite::from_image(texture),
            Transform::from_translation(position),
            GlobalTransform::default(),
            PieceVisual { owner, kind },
        ))
        .id()
}
pub fn spawn_piece_in_reserve(commands: &mut Commands, asset_server: &AssetServer, owner: Player,
    kind: PieceKind) -> Entity {
    let position = reserve_position(owner, kind);
    spawn_piece(commands, asset_server, owner, kind, position)
}

pub fn reserve_position(owner: Player, kind: PieceKind) -> Vec3 {
    let x = match owner {
        Player::White => WHITE_RESERVE_X,
        Player::Black => BLACK_RESERVE_X,
    };

    let y= RESERVE_LAYOUT
        .iter()
        .find(|slot| slot.kind == kind)
        .map(|slot| slot.y)
        .expect("Kind not in reserve layout");

    Vec3::new(x, y, PIECE_Z)
}

fn sprite_path(player: Player, kind: PieceKind) -> String {
    let color = match player {
        Player::White => "white",
        Player::Black => "black",
    };

    let name = match kind {
        PieceKind::Pawn => "pawn",
        PieceKind::Rook => "rook",
        PieceKind::Knight => "knight",
        PieceKind::Bishop => "bishop",
    };

    format!("sprites/{}_{}.png", color, name)
}

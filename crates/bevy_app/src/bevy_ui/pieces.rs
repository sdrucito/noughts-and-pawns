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
    spawn_side(&mut commands, asset_server.as_ref(), Player::White, WHITE_RESERVE_X);
    spawn_side(&mut commands, asset_server.as_ref(), Player::Black, BLACK_RESERVE_X);
}

fn spawn_side(commands: &mut Commands, asset_server: &AssetServer, player: Player, x: f32) {
    for slot in RESERVE_LAYOUT {
        let texture = asset_server.load(sprite_path(player, slot.kind));

        commands.spawn((
            Sprite::from_image(texture),
            Transform::from_xyz(x, slot.y, PIECE_Z),
            GlobalTransform::default(),
            PieceVisual {
                owner: player,
                kind: slot.kind,
            },
        ));
    }
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

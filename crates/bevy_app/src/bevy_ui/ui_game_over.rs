use bevy::prelude::*;
use game_core::game::player::Player;
use crate::AppState;
use crate::bevy_ui::constants::{
    NORMAL_BUTTON_COLOR,
    HOVERED_BUTTON_COLOR,
    PRESSED_BUTTON_COLOR,
};
use crate::bevy_ui::game_flow::GameOverEvent;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_game_over.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_ui)
            .add_systems(Update, game_over_buttons.run_if(in_state(AppState::GameOver)))
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_ui);
    }
}

#[derive(Component)]
struct GameOverUI;

#[derive(Component)]
struct BackToMenuButton;

#[derive(Resource)]
struct GameOverData {
    winner: Player,
}

fn handle_game_over(
    mut events: MessageReader<GameOverEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
) {
    for ev in events.read() {
        commands.insert_resource(GameOverData {
            winner: ev.winner,
        });

        next_state.set(AppState::GameOver);
    }
}

fn setup_game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_over: Res<GameOverData>,
) {
    let font = asset_server.load("fonts/Jacquard12-Regular.ttf");

    let message = match game_over.winner {
        Player::White => "White Wins!",
        Player::Black => "Black Wins!",
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
        GameOverUI,
    ))
        .with_children(|parent| {

            parent.spawn((
                Text::new(message),
                TextFont {
                    font: font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent.spawn((
                Button,
                Node {
                    width: Val::Px(260.0),
                    height: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON_COLOR),
                BackToMenuButton,
            ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Back to Main Menu"),
                        TextFont {
                            font,
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

fn game_over_buttons(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackToMenuButton>)
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut color) in &mut query {

        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(PRESSED_BUTTON_COLOR);
                next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *color = BackgroundColor(NORMAL_BUTTON_COLOR);
            }
        }
    }
}

fn cleanup_game_over_ui(
    mut commands: Commands,
    query: Query<Entity, With<GameOverUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<GameOverData>();
}
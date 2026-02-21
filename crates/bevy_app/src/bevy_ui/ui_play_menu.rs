use bevy::prelude::*;
use crate::AppState;
use crate::bevy_ui::constants::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::bevy_ui::ui_toasts::{spawn_toast, update_toasts};

pub struct PlayMenuPlugin;

impl Plugin for PlayMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::PlayMenu), setup_play_menu)
            .add_systems(Update,(play_menu_interactions, update_toasts)
                .run_if(in_state(AppState::PlayMenu)))
            .add_systems(OnExit(AppState::PlayMenu), cleanup_play_menu);
    }
}

#[derive(Component)]
struct PlayMenu;

#[derive(Component)]
enum PlayMenuButton {
    VsPlayer,
    VsAI,
    Back,
}

#[derive(Component)]
struct PlayAsBlackCheckbox {
    enabled: bool,
}

#[derive(Component)]
struct CheckboxSquare;

#[derive(Component)]
struct TimerSelector {
    index: usize,
}
const TIMER_VALUES: [&str; 5] = [
    "No Timer",
    "15 seconds",
    "30 seconds",
    "60 seconds",
    "120 seconds",
];
fn setup_play_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let heading_font = asset_server.load("fonts/Jacquard12-Regular.ttf");
    let simple_font = asset_server.load("fonts/MedodicaRegular.otf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            PlayMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Set game"),
                TextFont {
                    font: heading_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_play_button(parent, simple_font.clone(), "Vs Player", PlayMenuButton::VsPlayer);

            parent
                .spawn(Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(16.0),
                    ..default()
                })
                .with_children(|row| {
                    spawn_play_button(row, simple_font.clone(),"Vs AI", PlayMenuButton::VsAI);

                    row.spawn(Node {
                        width: Val::Auto,
                        height: Val::Auto,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(8.0),
                        ..default()
                    })
                        .with_children(|col| {

                            col.spawn((
                                Button,
                                Node {
                                    width: Val::Px(16.0),
                                    height: Val::Px(16.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(Color::BLACK),
                                PlayAsBlackCheckbox { enabled: false },
                                CheckboxSquare,
                            ));
                            col.spawn((
                                Text::new("Play as Black"),
                                TextFont {
                                    font: simple_font.clone(),
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(350.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    TimerSelector { index: 0 },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("<"),
                        TextFont {
                            font: simple_font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    parent.spawn((
                        Text::new(TIMER_VALUES[0]),
                        TextFont {
                            font: simple_font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                    parent.spawn((
                        Text::new(">"),
                        TextFont {
                            font: simple_font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            spawn_play_button(parent, simple_font.clone(), "Back", PlayMenuButton::Back);
        });
}

fn spawn_play_button(parent: &mut ChildSpawnerCommands, font: Handle<Font>, label: &str,
                     button_type: PlayMenuButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(256.0),
                height: Val::Px(64.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON_COLOR),
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font,
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn play_menu_interactions(
    mut button_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayMenuButton>,
            Option<&mut PlayAsBlackCheckbox>,
            Option<&mut TimerSelector>,
            Option<&Children>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>, mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands, asset_server: Res<AssetServer>) {

    for (entity, interaction, mut bg_color, play_button, checkbox, timer_selector, children) in
        &mut button_query
    {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BackgroundColor(PRESSED_BUTTON_COLOR);
            }
            Interaction::Hovered => {
                *bg_color = BackgroundColor(HOVERED_BUTTON_COLOR);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(NORMAL_BUTTON_COLOR);
            }
        }

        if *interaction != Interaction::Pressed {
            continue;
        }

        if let Some(button_type) = play_button {
            match button_type {
                PlayMenuButton::VsPlayer => {
                    info!("Starting Player vs Player");
                    next_state.set(AppState::InGame);
                }
                PlayMenuButton::VsAI => {
                    info!("Vs AI (not implemented yet)");
                    spawn_toast(&mut commands, &asset_server,"AI not implemented yet",1.5,0.8);
                }
                PlayMenuButton::Back => {
                    next_state.set(AppState::MainMenu);
                }
            }
            continue;
        }

        if let Some(mut checkbox) = checkbox {
            checkbox.enabled = !checkbox.enabled;
            info!("Play as Black toggled: {}", checkbox.enabled);
            continue;
        }

        if let Some(mut selector) = timer_selector {
            selector.index = (selector.index + 1) % TIMER_VALUES.len();
            let label_text = TIMER_VALUES[selector.index];

            if let Some(children) = children {
                if let Some(&label_entity) = children.get(1) {
                    if let Ok(mut text) = text_query.get_mut(label_entity) {
                        *text = Text::new(label_text);
                    }
                }
            }
            info!("Timer set to: {}", label_text);
        }
    }
}
fn update_checkbox_visuals(
    checkbox_query: Query<(&PlayAsBlackCheckbox, &Children)>,
    mut square_query: Query<&mut BackgroundColor, With<CheckboxSquare>>,
) {
    for (checkbox, children) in &checkbox_query {
        if let Some(&square_entity) = children.get(0) {
            if let Ok(mut square_bg) = square_query.get_mut(square_entity) {
                *square_bg = if checkbox.enabled {
                    BackgroundColor(Color::WHITE)
                } else {
                    BackgroundColor(Color::BLACK)
                };
            }
        }
    }
}
fn cleanup_play_menu(
    mut commands: Commands,
    query: Query<Entity, With<PlayMenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
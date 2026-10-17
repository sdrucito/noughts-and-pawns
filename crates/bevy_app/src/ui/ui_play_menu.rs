use bevy::asset::ErasedAssetLoader;
use bevy::prelude::*;
use crate::AppState;
use crate::shared::constants::{TEXT_COLOR1, TEXT_COLOR2};
use super::super::shared::constants::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::ui::ui_toasts::{spawn_toast, update_toasts};
use crate::ui::ui_button::{spawn_text_button, ButtonTextStyle};

pub struct PlayMenuPlugin;

impl Plugin for PlayMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::PlayMenu), setup_play_menu)
            .add_systems(Update,(play_menu_interactions, update_toasts, update_checkbox_visuals)
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
    let simple_font = asset_server.load("fonts/Fredoka-Bold.ttf");

    let dark_button_idle   = asset_server.load("ui/Button_idle.png");
    let dark_button_hover  = asset_server.load("ui/Button_hover.png");
    let dark_button_pressed = asset_server.load("ui/Button_press.png");

    let light_button_idle   = asset_server.load("ui/light_Button_idle.png");
    let light_button_hover  = asset_server.load("ui/light_Button_hover.png");
    let light_button_pressed = asset_server.load("ui/light_Button_press.png");

    let button_text_style1 = ButtonTextStyle {
        font_size: 30.0,
        color: TEXT_COLOR1,
        text_shadow: None,
    };
    let button_text_style2 = ButtonTextStyle {
        font_size: 30.0,
        color: TEXT_COLOR2,
        text_shadow: None,
    };

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
            // Heading
            parent.spawn((
                Text::new("Play"),
                TextFont {
                    font: heading_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Timer selector
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(192.0),
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

            // Two Players
            spawn_text_button(parent, simple_font.clone(), "Two Players", PlayMenuButton::VsPlayer,
                Node {
                    width: Val::Px(256.0),
                    height: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                &dark_button_idle, &dark_button_hover, &dark_button_pressed, button_text_style1.clone()
            );

            // Single Player
            spawn_text_button(parent, simple_font.clone(), "Single Player", PlayMenuButton::VsAI,
                Node {
                    width: Val::Px(256.0),
                    height: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                &dark_button_idle, &dark_button_hover, &dark_button_pressed, button_text_style1.clone()
            );

            // Play as Black option
            parent
                .spawn(Node {
                    width: Val::Px(256.0),
                    height: Val::Px(32.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(8.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Button,
                        Node {
                            width: Val::Px(16.0),
                            height: Val::Px(16.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                        PlayAsBlackCheckbox { enabled: false },
                        CheckboxSquare,
                    ));

                    parent.spawn((
                        Text::new("Play as Black"),
                        TextFont {
                            font: simple_font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Back
            spawn_text_button(parent, simple_font.clone(), "Back", PlayMenuButton::Back,
                Node {
                    width: Val::Px(256.0),
                    height: Val::Px(64.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::top(Val::Px(8.0)),
                    ..default()
                },
                &light_button_idle, &light_button_hover, &light_button_pressed, button_text_style2
            );
        });
}

fn play_menu_interactions(
        mut button_query: Query<
            (
                &Interaction,
                Option<&mut BackgroundColor>,
                Option<&PlayMenuButton>,
                Option<&mut PlayAsBlackCheckbox>,
                Option<&mut TimerSelector>,
                Option<&Children>,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut text_query: Query<&mut Text>,
        mut next_state: ResMut<NextState<AppState>>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        for (interaction, bg_color, play_button, checkbox, timer_selector, children)
        in &mut button_query
        {
            if let Some(mut bg_color) = bg_color {
                *bg_color = match *interaction {
                    Interaction::Pressed => BackgroundColor(PRESSED_BUTTON_COLOR),
                    Interaction::Hovered => BackgroundColor(HOVERED_BUTTON_COLOR),
                    Interaction::None => BackgroundColor(NORMAL_BUTTON_COLOR),
                };
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
    mut query: Query<(&PlayAsBlackCheckbox, &mut BackgroundColor),
        With<CheckboxSquare>>) {
    for (checkbox, mut bg) in &mut query {
        *bg = if checkbox.enabled {
            BackgroundColor(Color::WHITE)
        } else {
            BackgroundColor(Color::srgb(0.1,0.1,0.1))
        };
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
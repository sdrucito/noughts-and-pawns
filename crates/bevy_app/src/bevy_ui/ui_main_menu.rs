use bevy::prelude::*;
use crate::AppState;
use crate::bevy_ui::constants::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::bevy_ui::ui_toasts::{spawn_toast, update_toasts};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(Update,(menu_buttons, update_toasts)
                .run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    }
}
#[derive(Component)]
struct MainMenu;
#[derive(Component)]
enum MainMenuButton {
    Play,
    Options,
    Quit,
}
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {

    let heading_font = asset_server.load("fonts/Jacquard12-Regular.ttf");
    let simple_font = asset_server.load("fonts/MedodicaRegular.otf");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        MainMenu,
    ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Noughts & Pawns"),
                TextFont {
                    font: heading_font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            spawn_button(parent, simple_font.clone(), "Play", MainMenuButton::Play);
            spawn_button(parent, simple_font.clone(), "Options", MainMenuButton::Options);
            spawn_button(parent, simple_font.clone(), "Quit", MainMenuButton::Quit);
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, font: Handle<Font>, label: &str, button_type: MainMenuButton) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(256.0),
            height: Val::Px(64.0),
            margin: UiRect::all(Val::Px(30.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
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

fn menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button_type, mut bg_color) in &mut interaction_query {
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

        if *interaction == Interaction::Pressed {
            match button_type {
                MainMenuButton::Play => {
                    next_state.set(AppState::PlayMenu);
                }
                MainMenuButton::Options => {
                    spawn_toast(&mut commands, &asset_server, "Options not implemented yet", 1.5, 0.8); //TODO
                }
                MainMenuButton::Quit => {
                    std::process::exit(0);
                }
            }
        }
    }
}
fn cleanup_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

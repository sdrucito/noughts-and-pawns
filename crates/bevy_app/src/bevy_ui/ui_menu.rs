use bevy::prelude::*;
use crate::AppState;
use crate::bevy_ui::ui_toasts::{spawn_toast, update_toasts};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), setup_menu)
            .add_systems(Update,(menu_buttons, update_toasts)
                .run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}
#[derive(Component)]
struct MainMenu;
#[derive(Component)]
enum MenuButton {
    VsPlayer,
    VsAI,
}
#[derive(Component)]
struct MainMenuUI;
#[derive(Component)]
struct AiButtonToast {
    timer: Timer,
}
const NORMAL_BUTTON_COLOR: Color  = Color::srgb(0.3, 0.3, 0.3);
const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.2, 0.6, 0.9);
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

            spawn_button(parent, simple_font.clone(), "Play vs Player", MenuButton::VsPlayer);
            spawn_button(parent, simple_font.clone(), "Play vs AI", MenuButton::VsAI);
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, font: Handle<Font>, label: &str, button_type: MenuButton) {
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
        (&Interaction, &MenuButton, &mut BackgroundColor),
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
                MenuButton::VsPlayer => {
                    info!("Starting Player vs Player");
                    next_state.set(AppState::InGame);
                }
                MenuButton::VsAI => {
                    info!("AI not implemented yet");

                    spawn_toast(&mut commands, &asset_server, "AI not implemented yet", 1.5, 0.8);
                }
            }
        }
    }
}




fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

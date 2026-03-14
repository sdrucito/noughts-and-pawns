use bevy::prelude::*;
use bevy::sprite::Text2dShadow;
use crate::AppState;
use crate::bevy_ui::constants::{TEXT_COLOR};
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
#[derive(Component)]
struct ButtonVisuals {
    idle: Handle<Image>,
    hover: Handle<Image>,
    pressed: Handle<Image>,
}
fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {

    let heading_font = asset_server.load("fonts/Jacquard12-Regular.ttf");
    let simple_font = asset_server.load("fonts/MedodicaRegular.otf");

    let button_idle   = asset_server.load("ui/Button.png");
    let button_hover  = asset_server.load("ui/Button_hover.png");
    let button_pressed = asset_server.load("ui/Button_press.png");

    let bg= asset_server.load("ui/Background.png");
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
        //BackgroundColor(Color::srgb_u8(67,78,102)),
        ImageNode::new(bg.clone()),
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
                TextColor(TEXT_COLOR),
                TextShadow::default(),
            ));

            spawn_button(parent, simple_font.clone(), "Play", MainMenuButton::Play, &button_idle, &button_hover, &button_pressed);
            spawn_button(parent, simple_font.clone(), "Options", MainMenuButton::Options, &button_idle, &button_hover, &button_pressed);
            spawn_button(parent, simple_font.clone(), "Quit", MainMenuButton::Quit, &button_idle, &button_hover, &button_pressed);
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, font: Handle<Font>, label: &str, button_type: MainMenuButton,
                idle: &Handle<Image>, hover: &Handle<Image>, pressed: &Handle<Image>,) {
    parent.spawn((
        Button,
        Node {
            width: Val::Px(192.0),
            height: Val::Px(64.0),
            margin: UiRect::all(Val::Px(30.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ImageNode::new(idle.clone()),
        ButtonVisuals {
            idle: idle.clone(),
            hover: hover.clone(),
            pressed: pressed.clone(),
        },        button_type,
    ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font,
                    font_size: 30.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                TextShadow{
                    offset: Vec2::new(2.0,2.0),
                    ..default()
                }
            ));
        });
}

fn menu_buttons(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton, &ButtonVisuals, &mut ImageNode),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button_type, visuals, mut ui_image) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                ui_image.image = visuals.pressed.clone();
            }
            Interaction::Hovered => {
                ui_image.image = visuals.hover.clone();
            }
            Interaction::None => {
                ui_image.image = visuals.idle.clone();
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

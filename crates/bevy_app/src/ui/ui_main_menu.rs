use bevy::prelude::*;
use crate::AppState;
use crate::shared::constants::{TEXT_COLOR};
use crate::ui::ui_toasts::{spawn_toast, update_toasts};
use crate::ui::ui_button::{spawn_text_button, ButtonTextStyle};
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

    let button_idle   = asset_server.load("ui/Button.png");
    let button_hover  = asset_server.load("ui/Button_hover.png");
    let button_pressed = asset_server.load("ui/Button_press.png");

    let bg= asset_server.load("ui/Background.png");
    let button_text_style = ButtonTextStyle {
        font_size: 30.0,
        color: TEXT_COLOR,
        text_shadow: Some(TextShadow {
            offset: Vec2::new(2.0, 2.0),
            ..default()
        }),
    };

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

            spawn_text_button(parent, simple_font.clone(), "Play", MainMenuButton::Play, main_menu_button_node(),
                &button_idle, &button_hover, &button_pressed, button_text_style.clone());
            spawn_text_button(parent, simple_font.clone(), "Options", MainMenuButton::Options, main_menu_button_node(),
                &button_idle, &button_hover, &button_pressed, button_text_style.clone());
            spawn_text_button(parent, simple_font.clone(), "Quit", MainMenuButton::Quit, main_menu_button_node(),
                &button_idle, &button_hover, &button_pressed, button_text_style);
        });
}

fn main_menu_button_node() -> Node {
    Node {
        width: Val::Px(192.0),
        height: Val::Px(64.0),
        margin: UiRect::all(Val::Px(30.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

fn menu_buttons(
    interaction_query: Query<(&Interaction, &MainMenuButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button_type {
            MainMenuButton::Play => {
                next_state.set(AppState::PlayMenu);
            }
            MainMenuButton::Options => {
                spawn_toast(&mut commands, &asset_server, "Options not implemented yet", 1.5, 0.8, );
            }
            MainMenuButton::Quit => {
                std::process::exit(0);
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

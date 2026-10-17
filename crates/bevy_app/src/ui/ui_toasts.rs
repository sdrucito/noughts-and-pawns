use bevy::prelude::*;

#[derive(Component)]
pub struct Toast {
    hold_timer: Timer,
    fade_timer: Timer,
}

pub fn spawn_toast(commands: &mut Commands, asset_server: &AssetServer,
                   message: &str, hold: f32, fade: f32,
) {
    let font = asset_server.load("fonts/rainyhearts-edit.otf");

    commands
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                bottom: Val::Px(40.0),
                left: Val::Percent(50.0),
                border_radius: BorderRadius::all(px(10)),
                margin: UiRect {
                    left: Val::Px(-150.0),
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            Toast {
                hold_timer: Timer::from_seconds(hold, TimerMode::Once),
                fade_timer: Timer::from_seconds(fade, TimerMode::Once),
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(message),
                TextFont {
                    font,
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
pub fn update_toasts(time: Res<Time>, mut commands: Commands,
                            mut query: Query<(Entity, &mut Toast, &mut BackgroundColor, &Children)>,
                            mut text_query: Query<&mut TextColor>,
) {
    for (entity, mut toast, mut bg_color, children) in &mut query {
        if !toast.hold_timer.is_finished() {
            toast.hold_timer.tick(time.delta());
            continue;
        }

        toast.fade_timer.tick(time.delta());

        let progress = toast.fade_timer.fraction();
        //let alpha = 1.0 - progress; // linear
        let alpha = (1.0 - progress).powf(2.0);

        bg_color.0.set_alpha(alpha * 0.8);

        for child in children.iter() {
            if let Ok(mut text_color) = text_query.get_mut(child) {
                text_color.0.set_alpha(alpha);
            }
        }

        if toast.fade_timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
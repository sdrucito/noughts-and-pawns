
use bevy::prelude::*;

#[derive(Component)]
pub struct ButtonVisuals {
    pub idle: Handle<Image>,
    pub hover: Handle<Image>,
    pub pressed: Handle<Image>,
}

#[derive(Clone)]
pub struct ButtonTextStyle {
    pub font_size: f32,
    pub color: Color,
    pub text_shadow: Option<TextShadow>,
}

pub struct UIButtonPlugin;

impl Plugin for UIButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_button_visuals);
    }
}
pub fn spawn_text_button<T: Component>(
    parent: &mut ChildSpawnerCommands, font: Handle<Font>, label: &str, button_type: T, node: Node,
    idle: &Handle<Image>, hover: &Handle<Image>, pressed: &Handle<Image>, text_style: ButtonTextStyle
) {
    let ButtonTextStyle { font_size, color, text_shadow} = text_style;

    parent
        .spawn((
            Button,
            node,
            ImageNode::new(idle.clone()),
            ButtonVisuals {
                idle: idle.clone(),
                hover: hover.clone(),
                pressed: pressed.clone(),
            },
            button_type,
        ))
        .with_children(move |parent| {
            let mut text_entity = parent.spawn((
                Text::new(label),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(color),
            ));

            if let Some(shadow) = text_shadow {
                text_entity.insert(shadow);
            }
        });
}
pub fn spawn_icon_button<T: Component>(
    parent: &mut ChildSpawnerCommands, button_type: T, node: Node,
    idle: &Handle<Image>, hover: &Handle<Image>, pressed: &Handle<Image>
) {
    parent.spawn((
        Button,
        node,
        ImageNode::new(idle.clone()),
        ButtonVisuals {
            idle: idle.clone(),
            hover: hover.clone(),
            pressed: pressed.clone(),
        },
        button_type,
    ));
}

fn update_button_visuals(mut button_query: Query<(&Interaction, &ButtonVisuals, &mut ImageNode), (Changed<Interaction>, With<Button>)>) {
    for (interaction, visuals, mut image_node) in &mut button_query {
        image_node.image = match *interaction {
            Interaction::Pressed => visuals.pressed.clone(),
            Interaction::Hovered => visuals.hover.clone(),
            Interaction::None => visuals.idle.clone(),
        };
    }
}

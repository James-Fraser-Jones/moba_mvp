use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub fn init(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
                order: 1,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(1),
    ));
}

pub fn update() {}

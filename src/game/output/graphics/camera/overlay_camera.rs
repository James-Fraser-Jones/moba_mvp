//use crate::game::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub struct OverlayCameraPlugin;
impl Plugin for OverlayCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(OverlayCameraSet));
        app.add_systems(Update, update.in_set(OverlayCameraSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OverlayCameraSet;

fn init(mut commands: Commands) {
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

fn update() {}

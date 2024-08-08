//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::{os::Handles, *};
use bevy::{prelude::*, render::*};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

pub const WALL_HEIGHT: f32 = 30.;

#[derive(Component, Default)]
pub struct Map;

fn init(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: server.load("models/map.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(1., 1., WALL_HEIGHT)),
            ..default()
        },
        Map,
    ));
}

fn update() {}

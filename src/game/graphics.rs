//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::{os::Handles, *};
use bevy::gltf::GltfMesh;
use bevy::{math::Affine2, prelude::*, render::*};
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_resources, init).chain().after(os::init));
        app.add_systems(Update, update);
    }
}

const WALL_HEIGHT: f32 = 50.;

#[derive(Component, Default)]
pub struct Map;

fn init_resources() {}

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

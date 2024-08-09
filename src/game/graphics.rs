//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::{os::Handles, *};
use bevy::{color::palettes::css, prelude::*, render::*};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

pub const WALL_HEIGHT: f32 = 30.;
pub const BLENDER_WALL_HEIGHT: f32 = 50.;

#[derive(Component, Default)]
pub struct Map;

fn init(mut commands: Commands, server: Res<AssetServer>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::Srgba(css::FOREST_GREEN);
    commands.spawn((
        SceneBundle {
            scene: server.load("models/map.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(1., 1., WALL_HEIGHT / BLENDER_WALL_HEIGHT)),
            ..default()
        },
        Map,
    ));
}

fn update() {}

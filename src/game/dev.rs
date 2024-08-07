//responsibilities:
//enabling wireframe, gizmos, etc..
//stuff for purely-dev-related functionality

use crate::game::*;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WireframePlugin);
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update(
    mut wall_query: Query<&mut Transform, With<graphics::Wall>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
    if keyboard.just_pressed(KeyCode::KeyI) {
        for mut transform in &mut wall_query {
            transform.scale.z = transform.scale.z + (10. / graphics::BLENDER_WALL_HEIGHT);
        }
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        for mut transform in &mut wall_query {
            transform.scale.z = transform.scale.z - (10. / graphics::BLENDER_WALL_HEIGHT);
        }
    }
}

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
    mut map_query: Query<&mut Transform, With<graphics::Map>>,
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
    if keyboard.just_pressed(KeyCode::KeyI) {
        let mut transform = map_query.single_mut();
        transform.scale.z += 10.;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        let mut transform = map_query.single_mut();
        transform.scale.z -= 10.;
    }
}

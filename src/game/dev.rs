//responsibilities:
//enabling wireframe, gizmos, etc..
//stuff for purely-dev-related functionality

use super::*;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WireframePlugin, FpsOverlayPlugin::default()));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

pub const WIREFRAME_ENABLED: bool = false;

fn init(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = WIREFRAME_ENABLED;
}

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
        transform.scale.z += 10. / graphics::BLENDER_WALL_HEIGHT;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        let mut transform = map_query.single_mut();
        transform.scale.z -= 10. / graphics::BLENDER_WALL_HEIGHT;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        let mut transform = map_query.single_mut();
        transform.scale.z = graphics::WALL_HEIGHT / graphics::BLENDER_WALL_HEIGHT;
        wireframe_config.global = WIREFRAME_ENABLED;
    }
}

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
fn update(mut wireframe_config: ResMut<WireframeConfig>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        wireframe_config.global = WIREFRAME_ENABLED;
    }
}

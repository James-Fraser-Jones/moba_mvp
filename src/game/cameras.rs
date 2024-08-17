pub mod orbit_camera;
pub mod overlay_camera;

use super::*;
use bevy::prelude::*;

pub struct CamerasPlugin;
impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update.in_set(UpdateCameras::PreLogic));
    }
}

fn init() {}

fn update() {}

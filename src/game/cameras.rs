pub mod orbit_camera;
pub mod overlay_camera;

use bevy::prelude::*;

pub struct CamerasPlugin;
impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (orbit_camera::init, overlay_camera::init));
        app.add_systems(Update, (orbit_camera::update, overlay_camera::update));
    }
}

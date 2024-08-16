pub mod orbit_camera;
pub mod overlay_camera;

use bevy::color::palettes::css;
use bevy::prelude::*;

pub struct CamerasPlugin;
impl Plugin for CamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            orbit_camera::OrbitCameraPlugin,
            overlay_camera::OverlayCameraPlugin,
        ));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::Srgba(css::FOREST_GREEN);
}

fn update() {}

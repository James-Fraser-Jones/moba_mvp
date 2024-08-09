//responsibilities:
//enabling wireframe, gizmos, etc..
//stuff for purely-dev-related functionality

use crate::game::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WireframePlugin, FpsOverlayPlugin::default()));
        app.add_systems(Startup, init);
        app.add_systems(Update, (update, draw_cursor));
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
        transform.scale.z += 10.;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        let mut transform = map_query.single_mut();
        transform.scale.z -= 10.;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        let mut transform = map_query.single_mut();
        transform.scale.z = graphics::WALL_HEIGHT;
        wireframe_config.global = WIREFRAME_ENABLED;
    }
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    last_cursor_position: Res<input::LastCursorPosition>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_global_transform) = camera_query.single();
    let ray = camera
        .viewport_to_world(camera_global_transform, last_cursor_position.0)
        .unwrap();
    let distance = ray
        .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z))
        .unwrap();
    let point = ray.get_point(distance);
    gizmos.circle(
        point + Vec3::Z * 0.01,
        Dir3::new(Vec3::Z).unwrap(),
        10.,
        Color::WHITE,
    );
    gizmos.arrow(point + Vec3::Z * 30., point + Vec3::Z * 0.01, Color::WHITE);
}

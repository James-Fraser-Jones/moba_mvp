//responsibilities:
//enabling wireframe, gizmos, etc..
//stuff for purely-dev-related functionality

use crate::game::*;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WireframePlugin);
        app.add_systems(Startup, init);
        app.add_systems(Update, (update, draw_cursor));
    }
}

pub const WIREFRAME_ENABLED: bool = true;

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
    camera_query: Query<(&Camera, &GlobalTransform), With<camera::MainCameraMarker>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = window_query.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Z)) else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    let end = point + Vec3::Z * 0.01;
    gizmos.circle(end, Dir3::new(Vec3::Z).unwrap(), 10., Color::WHITE);
    gizmos.arrow(point + Vec3::Z * 50., end, Color::WHITE);
}

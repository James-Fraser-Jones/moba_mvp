//responsibilities:
//initializing useful camera abstraction, and associated camera settings
//allowing easy reset
//utilizing input plugin to enable movement, rotation, zoom, etc..

use crate::game::*;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

const CAMERA_DRAW_FAR: f32 = 2000.;
const PAN_SPEED: f32 = 400.;
const ZOOM_SPEED: f32 = 100.;
const DEBUG_CONTROLS: bool = false;
const ROTATION_SPEED: f32 = 0.15;

#[derive(Bundle)]
struct OrbitCamera3dBundle {
    camera_3d_bundle: Camera3dBundle,
    orbit_transform: OrbitTransform,
}
impl Default for OrbitCamera3dBundle {
    fn default() -> Self {
        Self {
            camera_3d_bundle: Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    far: CAMERA_DRAW_FAR,
                    ..default()
                }),
                ..default()
            },
            orbit_transform: OrbitTransform::default(),
        }
    }
}

#[derive(Component, Clone, Copy)]
struct OrbitTransform {
    position: Vec3,
    rotation: Vec2,
    orbit_distance: f32,
}
impl Default for OrbitTransform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Vec2::new(0., 0.6),
            orbit_distance: 340.,
        }
    }
}
// impl OrbitTransform {
//     fn from_transform(&self, transform: Transform) -> OrbitTransform {
//         OrbitTransform {
//             position: ,
//             rotation: ,
//             orbit_distance: self.orbit_distance,
//         }
//     }
// }
impl From<OrbitTransform> for Transform {
    fn from(orbit_transform: OrbitTransform) -> Self {
        let rotation = Quat::from_euler(
            EulerRot::ZYX,
            orbit_transform.rotation.x,
            0.,
            orbit_transform.rotation.y,
        );
        Transform {
            translation: orbit_transform.position
                + rotation.mul_vec3(Vec3::Z * orbit_transform.orbit_distance),
            rotation,
            ..default()
        }
    }
}

fn init(mut commands: Commands) {
    commands.spawn(OrbitCamera3dBundle::default());
}

fn update(
    keyboard_axis: Res<input::KeyboardAxis>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mouse_axis: Res<input::MouseAxis>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    wheel_axis: Res<input::WheelAxis>,
    screen_axis: Res<input::ScreenAxis>,
    mut camera_query: Query<(&mut Transform, &mut OrbitTransform)>,
) {
    let (mut transform, mut orbit_transform) = camera_query.single_mut();
    if DEBUG_CONTROLS {
        //rotate
        if mouse_buttons.pressed(MouseButton::Left) {
            orbit_transform.rotation =
                (orbit_transform.rotation - mouse_axis.0 * ROTATION_SPEED) % (2. * PI);
            orbit_transform.rotation.y = orbit_transform.rotation.y.clamp(0., PI / 2.);
        }
        //pan (incl vertically)
        let yaw = orbit_transform.rotation.x;
        orbit_transform.position +=
            Quat::from_rotation_z(yaw).mul_vec3(keyboard_axis.0 * PAN_SPEED);
        //zoom
        orbit_transform.orbit_distance -= wheel_axis.0.y * ZOOM_SPEED;
        orbit_transform.orbit_distance = orbit_transform.orbit_distance.max(0.);
    } else {
        //pan
        let yaw = orbit_transform.rotation.x;
        orbit_transform.position +=
            Quat::from_rotation_z(yaw).mul_vec3((screen_axis.0 * PAN_SPEED).extend(0.));
        //zoom
        orbit_transform.orbit_distance -= wheel_axis.0.y * ZOOM_SPEED;
        orbit_transform.orbit_distance = orbit_transform.orbit_distance.max(1.);
        //center camera
        if keyboard_buttons.pressed(KeyCode::Space) {
            orbit_transform.position = Vec3::ZERO;
        }
    }
    //reset
    if keyboard_buttons.pressed(KeyCode::KeyR) {
        *orbit_transform = OrbitTransform::default();
    }
    *transform = Transform::from(*orbit_transform);
}

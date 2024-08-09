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
    orbit_distance: OrbitDistance,
}
impl Default for OrbitCamera3dBundle {
    fn default() -> Self {
        let orbit_distance = OrbitDistance::default();
        Self {
            camera_3d_bundle: Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    far: CAMERA_DRAW_FAR,
                    ..default()
                }),
                transform: orbit_distance.orbit_transform_to_transform(&OrbitTransform::default()),
                ..default()
            },
            orbit_distance,
        }
    }
}

#[derive(Clone, Copy)]
struct OrbitTransform {
    translation: Vec3,
    rotation: Vec2,
}
impl Default for OrbitTransform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Vec2::new(0., 0.6),
        }
    }
}

#[derive(Component, Clone, Copy)]
struct OrbitDistance(f32);
impl Default for OrbitDistance {
    fn default() -> Self {
        Self(340.)
    }
}
impl OrbitDistance {
    fn transform_to_orbit_transform(&self, transform: &Transform) -> OrbitTransform {
        let rotation = transform.rotation.to_euler(EulerRot::ZYX);
        OrbitTransform {
            rotation: Vec2::new(rotation.0, rotation.2),
            translation: transform.translation - transform.rotation.mul_vec3(Vec3::Z * self.0),
        }
    }
    fn orbit_transform_to_transform(&self, orbit_transform: &OrbitTransform) -> Transform {
        let rotation = Quat::from_euler(
            EulerRot::ZYX,
            orbit_transform.rotation.x,
            0.,
            orbit_transform.rotation.y,
        );
        Transform {
            translation: orbit_transform.translation + rotation.mul_vec3(Vec3::Z * self.0),
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
    mut camera_query: Query<(&mut Transform, &mut OrbitDistance)>,
) {
    let (mut transform, mut orbit_distance) = camera_query.single_mut();
    let mut orbit_transform = orbit_distance.transform_to_orbit_transform(&transform);
    if DEBUG_CONTROLS {
        //rotate (both axes)
        if mouse_buttons.pressed(MouseButton::Middle) {
            orbit_transform.rotation =
                (orbit_transform.rotation - mouse_axis.0 * ROTATION_SPEED) % (2. * PI);
            orbit_transform.rotation.y = orbit_transform.rotation.y.clamp(0., PI / 2.);
        }
        //pan (incl vertically)
        let yaw = orbit_transform.rotation.x;
        orbit_transform.translation +=
            Quat::from_rotation_z(yaw).mul_vec3(keyboard_axis.0 * PAN_SPEED);
        //zoom
        orbit_distance.0 -= wheel_axis.0.y * ZOOM_SPEED;
        orbit_distance.0 = orbit_distance.0.max(0.);
    } else {
        //rotate
        if mouse_buttons.pressed(MouseButton::Middle) {
            orbit_transform.rotation =
                (orbit_transform.rotation - mouse_axis.0 * ROTATION_SPEED) % (2. * PI);
            orbit_transform.rotation.x = 0.;
            orbit_transform.rotation.y = orbit_transform.rotation.y.clamp(0., PI / 2.);
        }
        //pan
        let yaw = orbit_transform.rotation.x;
        orbit_transform.translation +=
            Quat::from_rotation_z(yaw).mul_vec3((screen_axis.0 * PAN_SPEED).extend(0.));
        //zoom
        orbit_distance.0 -= wheel_axis.0.y * ZOOM_SPEED;
        orbit_distance.0 = orbit_distance.0.max(0.);
        //center camera
        if keyboard_buttons.pressed(KeyCode::Space) {
            orbit_transform.translation = Vec3::ZERO;
        }
        //flip camera
        if keyboard_buttons.just_pressed(KeyCode::KeyQ) {
            orbit_transform.rotation.x = (orbit_transform.rotation.x + PI) % (2. * PI);
        }
    }
    //reset
    if keyboard_buttons.pressed(KeyCode::KeyR) {
        *orbit_distance = OrbitDistance::default();
        orbit_transform = OrbitTransform::default();
    }
    *transform = orbit_distance.orbit_transform_to_transform(&orbit_transform);
}

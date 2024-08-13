//responsibilities:
//initializing useful camera abstraction, and associated camera settings
//allowing easy reset
//utilizing input plugin to enable movement, rotation, zoom, etc..

use super::super::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::f32::consts::PI;

const CAMERA_DRAW_FAR: f32 = 2000.;
const PAN_SPEED: f32 = 450.;
const ZOOM_MIN: f32 = 20.;
const ZOOM_MAX: f32 = 2000.;
const ZOOM_SPEED: f32 = 100.;
const ROTATION_SPEED: f32 = 0.15;
const FLIP_ORIENTATION_SPEED: f32 = 5. * PI;

#[derive(Default)]
pub struct FlipOrientation(Option<f32>);

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
            rotation: Vec2::new(0., 0.5),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct OrbitDistance(f32);
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
    pub fn zoom(&self) -> f32 {
        OrbitDistance::default().0 / self.0
    }
}

pub fn init(mut commands: Commands) {
    commands.spawn((OrbitCamera3dBundle::default(), RenderLayers::layer(0)));
}

pub fn update(
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mouse_axis: Res<input::MouseAxis>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    wheel_axis: Res<input::WheelAxis>,
    screen_axis: Res<input::ScreenAxis>,
    mut camera_query: Query<(&mut Transform, &mut OrbitDistance)>,
    mut flip_orientation: Local<FlipOrientation>,
    time: Res<Time>,
) {
    let (mut transform, mut orbit_distance) = camera_query.single_mut();
    let mut orbit_transform = orbit_distance.transform_to_orbit_transform(&transform);
    //rotate
    if mouse_buttons.pressed(MouseButton::Middle) {
        orbit_transform.rotation.y -= mouse_axis.0.y * ROTATION_SPEED;
        orbit_transform.rotation.y = orbit_transform.rotation.y.clamp(0., PI / 2. - 0.01);
    }
    //pan
    let yaw = orbit_transform.rotation.x;
    orbit_transform.translation +=
        Quat::from_rotation_z(yaw).mul_vec3((screen_axis.0 * PAN_SPEED).extend(0.));
    orbit_transform.translation = orbit_transform
        .translation
        .clamp(Vec3::splat(-1000.), Vec3::splat(1000.));
    //zoom
    orbit_distance.0 -= wheel_axis.0.y * ZOOM_SPEED;
    orbit_distance.0 = orbit_distance.0.clamp(ZOOM_MIN, ZOOM_MAX);
    //flip orientation
    if keyboard_buttons.just_pressed(KeyCode::KeyQ) {
        if flip_orientation.0 == None {
            flip_orientation.0 = Some(orbit_transform.rotation.x + PI);
        }
    }
    if let Some(x) = flip_orientation.0 {
        let flip_delta = FLIP_ORIENTATION_SPEED * time.delta_seconds();
        if orbit_transform.rotation.x + flip_delta >= x {
            orbit_transform.rotation.x = x;
            flip_orientation.0 = None;
        } else {
            orbit_transform.rotation.x += flip_delta;
        }
    }
    //reset
    if keyboard_buttons.pressed(KeyCode::KeyR) {
        *orbit_distance = OrbitDistance::default();
        orbit_transform = OrbitTransform::default();
        flip_orientation.0 = None;
    }
    *transform = orbit_distance.orbit_transform_to_transform(&orbit_transform);
}

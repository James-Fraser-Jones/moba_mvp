//responsibilities:
//initializing useful camera abstraction, and associated camera settings
//allowing easy reset
//utilizing input plugin to enable movement, rotation, zoom, etc..

use super::{super::*, *};
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use std::f32::consts::PI;

pub struct OrbitCameraPlugin;
impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (
                update_camera.in_set(UpdateCameras::PreLogic),
                update_camera_post_logic.in_set(UpdateCameras::PostLogic),
            ),
        );
    }
}

const CAMERA_DRAW_FAR: f32 = 2000.;
const PITCH_SPEED: f32 = 0.15;
const FLIP_YAW_SPEED: f32 = 5. * PI;
const PAN_SPEED: f32 = 450.;
const ZOOM_SPEED: f32 = 0.1;
const ZOOM_MIN: f32 = 20.;
const ZOOM_MAX: f32 = 2000.;

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
    translation: Vec2,
    rotation: Vec2,
}
impl Default for OrbitTransform {
    fn default() -> Self {
        Self {
            translation: Vec2::splat(-800.),
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
            translation: (transform.translation - transform.rotation.mul_vec3(Vec3::Z * self.0))
                .truncate(),
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
            translation: orbit_transform.translation.extend(0.)
                + rotation.mul_vec3(Vec3::Z * self.0),
            rotation,
            ..default()
        }
    }
    pub fn zoom(&self) -> f32 {
        OrbitDistance::default().0 / self.0
    }
}

fn init(mut commands: Commands) {
    commands.spawn((OrbitCamera3dBundle::default(), RenderLayers::layer(0)));
}

fn update_camera(
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mouse_axis: Res<input::MouseAxis>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    wheel_axis: Res<input::WheelAxis>,
    screen_axis: Res<input::ScreenAxis>,
    mut camera_query: Query<(&mut Transform, &mut GlobalTransform, &mut OrbitDistance)>,
    mut flip_orientation: Local<FlipOrientation>,
    time: Res<Time>,
) {
    let (mut transform, mut global_transform, mut orbit_distance) = camera_query.single_mut();
    let mut orbit_transform = orbit_distance.transform_to_orbit_transform(&transform);

    //reset pitch and zoom
    if keyboard_buttons.just_pressed(KeyCode::KeyR) {
        *orbit_distance = OrbitDistance::default();
        orbit_transform.rotation.y = OrbitTransform::default().rotation.y;
    }
    //trigger yaw flipping
    if keyboard_buttons.just_pressed(KeyCode::KeyQ) {
        if flip_orientation.0 == None {
            flip_orientation.0 = Some(orbit_transform.rotation.x + PI);
        }
    }

    //adjust pitch
    if mouse_buttons.pressed(MouseButton::Middle) {
        orbit_transform.rotation.y -= mouse_axis.0.y * PITCH_SPEED;
        orbit_transform.rotation.y = orbit_transform.rotation.y.clamp(0., PI / 2. - 0.01);
    }
    //adjust yaw
    if let Some(x) = flip_orientation.0 {
        let flip_delta = FLIP_YAW_SPEED * time.delta_seconds();
        if orbit_transform.rotation.x + flip_delta >= x {
            orbit_transform.rotation.x = x;
            flip_orientation.0 = None;
        } else {
            orbit_transform.rotation.x += flip_delta;
        }
    }
    //pan
    let yaw = orbit_transform.rotation.x;
    orbit_transform.translation += Quat::from_rotation_z(yaw)
        .mul_vec3((screen_axis.0 * PAN_SPEED).extend(0.))
        .truncate();
    orbit_transform.translation = orbit_transform
        .translation
        .clamp(Vec2::splat(-1000.), Vec2::splat(1000.));
    //zoom
    orbit_distance.0 *= (1. + ZOOM_SPEED).powf(-wheel_axis.0.y);
    orbit_distance.0 = orbit_distance.0.clamp(ZOOM_MIN, ZOOM_MAX);

    //update transforms
    *transform = orbit_distance.orbit_transform_to_transform(&orbit_transform);
    *global_transform = GlobalTransform::from(*transform); //manually update global transform, ahead of transform propagation
}

fn update_camera_post_logic(
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<
        (&mut Transform, &mut GlobalTransform, &OrbitDistance),
        With<OrbitDistance>,
    >,
    player: Res<player::Player>,
    player_query: Query<&Transform, Without<OrbitDistance>>,
) {
    let (mut transform, mut global_transform, orbit_distance) = camera_query.single_mut();
    let mut orbit_transform = orbit_distance.transform_to_orbit_transform(&transform);
    let player = player_query.get(player.0).unwrap();
    if keyboard_buttons.pressed(KeyCode::Space) {
        orbit_transform.translation = player.translation.truncate();
        *transform = orbit_distance.orbit_transform_to_transform(&orbit_transform);
        *global_transform = GlobalTransform::from(*transform); //manually update global transform, ahead of transform propagation
    }
}

//these functions should be used after camera update (either pre or post logic) to get correct global transform

//logical pixels, top-left (0,0), to Vec2 representing intersection point with horizontal plane of height, in world space
pub fn pixel_to_horizontal_plane(
    pixel: Vec2,
    height: f32,
    camera: &Camera,
    transform: &GlobalTransform,
) -> Option<Vec2> {
    let pixel_ray = camera.viewport_to_world(transform, pixel).unwrap();
    let intersection_distance =
        pixel_ray.intersect_plane(Vec3::Z * height, InfinitePlane3d::new(Vec3::Z))?;
    let intersection_point = pixel_ray.get_point(intersection_distance);
    Some(intersection_point.truncate())
}

//inverse
pub fn position_to_pixel(
    position: Vec3,
    camera: &Camera,
    transform: &GlobalTransform,
) -> Option<Vec2> {
    camera.world_to_viewport(transform, position)
}

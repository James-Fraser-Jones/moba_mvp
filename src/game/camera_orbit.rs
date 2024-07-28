use crate::game::consts::*;
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    math::VectorSpace,
    prelude::*,
};
use std::f32::consts::PI;
use std::ops::DerefMut;

pub struct CameraOrbitPlugin;

impl Plugin for CameraOrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(Update, update_camera_fly);
    }
}

#[derive(Component, Default)]
struct MainCamera;

fn inital_transform() -> Transform {
    let mut initial = Transform::from_xyz(0., 0., 1000. / (FOV / 2.).tan());
    initial.rotate_around(Vec3::ZERO, Quat::from_rotation_x(PI / 8.));
    initial.looking_at(Vec3::ZERO, Vec3::Y)
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            projection: Projection::from(PerspectiveProjection {
                fov: FOV,
                far: 1000. / (FOV / 2.).tan() + 10.,
                ..default()
            }),
            transform: inital_transform(),
            ..default()
        },
        MainCamera,
    ));
}

fn update_camera_fly(
    mut query: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    mut mouse: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let (mut transform, mut projection) = query.single_mut();

    //reset position
    if keyboard.pressed(KeyCode::KeyR) {
        match projection.deref_mut() {
            Projection::Perspective(ref mut projection) => {
                projection.fov = FOV;
            }
            Projection::Orthographic(_) => {}
        };
        *transform = inital_transform();
    }

    //orbit
    let mut rot: Vec2 = Vec2::ZERO;
    for ev in evr_motion.read() {
        rot -= ev.delta;
    }
    rot *= CAMERA_TURN_SPEED * time.delta_seconds();
    //TODO finish later

    //pan
    let mut direction: Vec2 = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    direction = direction.normalize_or_zero();
    direction *= CAMERA_SPEED * time.delta_seconds();
    transform.translation += direction.extend(0.);

    //zoom
    for scroll_event in mouse.read() {
        if scroll_event.unit == MouseScrollUnit::Line {
            match projection.deref_mut() {
                Projection::Perspective(ref mut projection) => {
                    projection.fov -= scroll_event.y * PERSPECTIVE_ZOOM_SPEED
                }
                Projection::Orthographic(_) => {}
            }
        }
        //pinch zoom unsupported because mobas use mice
    }
}

use crate::game::consts::*;
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};
use std::f32::consts::PI;
use std::ops::DerefMut;

pub struct CameraPerspectivePlugin;

impl Plugin for CameraPerspectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(Update, update_camera_fly);
    }
}

#[derive(Component, Default)]
struct MainCamera;

fn inital_transform() -> Transform {
    let initial = Transform::from_xyz(0., 0., 1000. / (CAMERA_ZOOM / 2.).tan());
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
                fov: CAMERA_ZOOM,
                far: 1000. / (CAMERA_ZOOM / 2.).tan() + 10.,
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

    //reset position and zoom
    if keyboard.pressed(KeyCode::KeyR) {
        match projection.deref_mut() {
            Projection::Perspective(ref mut projection) => {
                projection.fov = CAMERA_ZOOM;
            }
            Projection::Orthographic(_) => {}
        };
        *transform = inital_transform();
    }

    //rotate
    let mut rot: Vec2 = Vec2::ZERO;
    for ev in evr_motion.read() {
        rot -= ev.delta;
    }
    rot *= CAMERA_TURN_SPEED * time.delta_seconds();
    transform.rotate_z(rot.x);
    transform.rotate_local_x(rot.y);

    //clamp x rotation
    let mut angles = transform.rotation.to_euler(EulerRot::ZYX);
    angles.2 -= 0.01;
    if angles.2 < -PI / 2. {
        angles.2 = PI;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, angles.0, angles.1, angles.2);
    } else if angles.2 < 0. {
        angles.2 = 0.;
        transform.rotation = Quat::from_euler(EulerRot::ZYX, angles.0, angles.1, angles.2);
    }

    //translate
    let mut direction: Vec3 = Vec3::ZERO;
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
    if keyboard.pressed(KeyCode::Space) {
        direction.z += 1.;
    }
    if keyboard.pressed(KeyCode::ControlLeft) {
        direction.z -= 1.;
    }
    direction = direction.normalize_or_zero();
    direction *= CAMERA_SPEED * time.delta_seconds();
    let direction_xy = transform
        .local_x()
        .as_vec3()
        .truncate()
        .rotate(direction.truncate())
        .extend(0.);
    transform.translation += direction_xy;
    transform.translation.z += direction.z;

    //zoom
    for scroll_event in mouse.read() {
        if scroll_event.unit == MouseScrollUnit::Line {
            match projection.deref_mut() {
                Projection::Perspective(ref mut projection) => {
                    projection.fov -= scroll_event.y * CAMERA_ZOOM_SPEED
                }
                Projection::Orthographic(_) => {}
            }
        }
        //pinch zoom unsupported because mobas use mice
    }
}

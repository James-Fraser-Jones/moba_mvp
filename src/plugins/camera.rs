use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::lib::consts::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component, Default)]
struct MainCamera;

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}

fn update_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    mut mouse: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut projection) = query.single_mut(); //okay when entity known to exist and be unique
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
    for scroll_event in mouse.read() {
        if scroll_event.unit == MouseScrollUnit::Line {
            projection.scale -= scroll_event.y * ZOOM_SPEED;
        }
        //pinch zoom unsupported because mobas use mice
    }
}

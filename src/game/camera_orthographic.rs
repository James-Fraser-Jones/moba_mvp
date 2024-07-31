use crate::game::consts::*;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    render::{
        camera::{OrthographicProjection, ScalingMode},
        view::RenderLayers,
    },
};
use std::ops::DerefMut;

pub struct CameraOrthographicPlugin;

impl Plugin for CameraOrthographicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_camera);
        app.add_systems(Update, update_camera_pan);
    }
}

#[derive(Component, Default)]
struct MainCamera;

#[derive(Component, Default)]
struct OverlayCamera;

fn inital_transform() -> Transform {
    let initial = Transform::from_xyz(0., 0., 1000. / (CAMERA_ZOOM / 2.).tan());
    initial.looking_at(Vec3::ZERO, Vec3::Y)
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                order: 0,
                ..default()
            },
            projection: Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(2000.),
                far: 1000. / (CAMERA_ZOOM / 2.).tan() + 10.,
                ..default()
            }),
            transform: inital_transform(),
            ..default()
        },
        RenderLayers::layer(0),
        MainCamera,
    ));
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::None,
                order: 1,
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(2000.),
                far: 1000. / (CAMERA_ZOOM / 2.).tan() + 10.,
                ..default()
            },
            transform: inital_transform(),
            ..default()
        },
        RenderLayers::layer(1),
        OverlayCamera,
    ));
}

fn update_camera_pan(
    mut query: Query<(&mut Transform, &mut Projection), With<MainCamera>>,
    mut mouse: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut projection) = query.single_mut(); //okay when entity known to exist and be unique

    //reset position and zoom
    if keyboard.pressed(KeyCode::KeyR) {
        match projection.deref_mut() {
            Projection::Orthographic(ref mut projection) => {
                projection.scale = 1.;
            }
            Projection::Perspective(_) => {}
        }
        *transform = inital_transform();
    }

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
                Projection::Orthographic(ref mut projection) => {
                    projection.scale -= scroll_event.y * CAMERA_ZOOM_SPEED
                }
                Projection::Perspective(_) => {}
            }
        }
        //pinch zoom unsupported because mobas use mice
    }
}

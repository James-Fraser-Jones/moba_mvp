//responsibilities:
//initializing useful camera abstraction, and associated camera settings
//exposing camera, and settings, as a resource
//allowing easy reset
//utilizing input plugin to enable movement, rotation, zoom, etc..

use crate::game::*;
use bevy::{prelude::*, render::view::RenderLayers};
use std::f32::consts::PI;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainCamera>();
        app.add_systems(Startup, init.after(os::init));
        app.add_systems(Update, (update_camera, sync_camera, sync_window));
    }
}

const NEAR: f32 = 0.1;
const FAR: f32 = 2000.;
const TRANSLATION_SPEED: f32 = 800.;
const ROTATION_SPEED: f32 = 0.15;
const DEPTH_SPEED: f32 = 300.;
const FOV_SPEED: f32 = 0.1;

#[derive(Resource)]
struct MainCamera {
    translation: Vec3,
    rotation: Vec2,
    depth: f32,
    fov: f32,
}
impl Default for MainCamera {
    fn default() -> Self {
        let fov = PI / 4.;
        Self {
            translation: Vec3::new(0., -150., 0.),
            rotation: Vec2::new(0., PI / 8.),
            depth: 1000. / (fov / 2.).tan(),
            fov,
        }
    }
}

#[derive(Component)]
struct MainCameraMarker;

fn init(
    mut commands: Commands,
    camera_settings: Res<MainCamera>,
    main_window: Res<os::MainWindow>,
) {
    commands
        .spawn((TransformBundle::default(), MainCameraMarker))
        .with_children(|builder| {
            builder
                .spawn(TransformBundle::default())
                .with_children(|builder| {
                    builder
                        .spawn(TransformBundle::default())
                        .with_children(|builder| {
                            builder.spawn((
                                Camera3dBundle {
                                    camera: Camera {
                                        clear_color: ClearColorConfig::Custom(Color::BLACK),
                                        order: 0,
                                        ..default()
                                    },
                                    projection: Projection::Perspective(PerspectiveProjection {
                                        aspect_ratio: main_window.aspect_ratio(),
                                        near: NEAR,
                                        far: FAR,
                                        ..default()
                                    }),
                                    ..default()
                                },
                                RenderLayers::layer(0),
                            ));
                        });
                });
        });
}

fn update_camera(
    keyboard_axis: Res<input::KeyboardAxis>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mouse_axis: Res<input::MouseAxis>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    wheel_axis: Res<input::WheelAxis>,
    mut camera_settings: ResMut<MainCamera>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        //rotation
        camera_settings.rotation =
            (camera_settings.rotation - mouse_axis.0 * ROTATION_SPEED) % (2. * PI);
        //clamp pitch
        camera_settings.rotation.y = camera_settings.rotation.y.clamp(0., PI / 2.);
    } else if mouse_buttons.pressed(MouseButton::Middle) {
        //depth
        camera_settings.depth += mouse_axis.0.y * DEPTH_SPEED;
        camera_settings.depth = camera_settings.depth.max(0.);
    }

    //translation
    let yaw = camera_settings.rotation.x;
    let speed = TRANSLATION_SPEED;
    camera_settings.translation += Quat::from_rotation_z(yaw).mul_vec3(keyboard_axis.0 * speed);

    //fov
    camera_settings.fov -= wheel_axis.0.y * FOV_SPEED;

    //reset
    if keyboard_buttons.pressed(KeyCode::KeyR) {
        *camera_settings = MainCamera::default();
    }
}

fn sync_camera(
    camera_settings: Res<MainCamera>,
    base_query: Query<Entity, With<MainCameraMarker>>,
    children_query: Query<&Children>,
    mut transform_query: Query<&mut Transform>,
    mut projection_query: Query<&mut Projection>,
) {
    let base_entity = base_query.single();
    let mut descendents = children_query.iter_descendants(base_entity);

    let pivot_entity = descendents.next().unwrap();
    let stick_entity = descendents.next().unwrap();
    let camera_entity = descendents.next().unwrap();

    let mut base_transform = transform_query.get_mut(base_entity).unwrap();
    base_transform.translation = camera_settings.translation;
    base_transform.rotation = Quat::from_rotation_z(camera_settings.rotation.x);

    let mut stick_transform = transform_query.get_mut(stick_entity).unwrap();
    stick_transform.translation = Vec3::ZERO.with_z(camera_settings.depth);

    let mut pivot_transform = transform_query.get_mut(pivot_entity).unwrap();
    pivot_transform.rotation = Quat::from_rotation_x(camera_settings.rotation.y);

    let mut camera_projection = projection_query.get_mut(camera_entity).unwrap();
    if let Projection::Perspective(ref mut projection) = *camera_projection {
        projection.fov = camera_settings.fov;
    };
}

fn sync_window(main_window: Res<os::MainWindow>, mut projection_query: Query<&mut Projection>) {
    if main_window.is_changed() {
        let mut camera_projection = projection_query.single_mut();
        if let Projection::Perspective(ref mut projection) = *camera_projection {
            projection.aspect_ratio = main_window.aspect_ratio();
        };
    }
}

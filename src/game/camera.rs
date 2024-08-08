//responsibilities:
//initializing useful camera abstraction, and associated camera settings
//exposing camera, and settings, as a resource
//allowing easy reset
//utilizing input plugin to enable movement, rotation, zoom, etc..

use crate::game::*;
use bevy::{prelude::*, render::view::RenderLayers, window::PrimaryWindow};
use std::f32::consts::PI;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainCamera>();
        app.add_systems(Startup, init);
        app.add_systems(Update, (update_camera, sync_camera));
    }
}

const NEAR: f32 = 0.1;
const FAR: f32 = 2000.;
const TRANSLATION_SPEED: f32 = 400.;
const ROTATION_SPEED: f32 = 0.15;
const DEPTH_SPEED: f32 = 100.;
const FOV_SPEED: f32 = 0.1;
const DEBUG_CONTROLS: bool = false;

#[derive(Resource, Debug)]
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
            translation: Vec3::new(0., 0., 0.),
            rotation: Vec2::new(0., 0.6),
            depth: 340.,
            fov,
        }
    }
}

#[derive(Component)]
struct MainCameraBaseMarker;
#[derive(Component)]
pub struct MainCameraMarker;

fn init(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    let aspect_ratio = os::aspect_ratio(window);
    commands
        .spawn((TransformBundle::default(), MainCameraBaseMarker))
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
                                        aspect_ratio,
                                        near: NEAR,
                                        far: FAR,
                                        ..default()
                                    }),
                                    ..default()
                                },
                                RenderLayers::layer(0),
                                MainCameraMarker,
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
    screen_axis: Res<input::ScreenAxis>,
    mut main_camera: ResMut<MainCamera>,
) {
    if DEBUG_CONTROLS {
        //rotation
        if mouse_buttons.pressed(MouseButton::Left) {
            main_camera.rotation =
                (main_camera.rotation - mouse_axis.0 * ROTATION_SPEED) % (2. * PI);
            main_camera.rotation.y = main_camera.rotation.y.clamp(0., PI / 2.); //clamp pitch
        }
        //fov
        else if mouse_buttons.pressed(MouseButton::Middle) {
            main_camera.fov += mouse_axis.0.y * FOV_SPEED;
        }
        //translation
        let yaw = main_camera.rotation.x;
        main_camera.translation +=
            Quat::from_rotation_z(yaw).mul_vec3(keyboard_axis.0 * TRANSLATION_SPEED);
        //depth
        main_camera.depth -= wheel_axis.0.y * DEPTH_SPEED;
        main_camera.depth = main_camera.depth.max(0.);
    } else {
        //translation
        let yaw = main_camera.rotation.x;
        main_camera.translation +=
            Quat::from_rotation_z(yaw).mul_vec3((screen_axis.0 * TRANSLATION_SPEED).extend(0.));
        //depth
        main_camera.depth -= wheel_axis.0.y * DEPTH_SPEED;
        main_camera.depth = main_camera.depth.max(1.);
        //center camera
        if keyboard_buttons.pressed(KeyCode::Space) {
            main_camera.translation = Vec3::ZERO;
        }
    }

    //reset
    if keyboard_buttons.pressed(KeyCode::KeyR) {
        *main_camera = MainCamera::default();
    }
}

fn sync_camera(
    main_camera: Res<MainCamera>,
    base_query: Query<Entity, With<MainCameraBaseMarker>>,
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
    base_transform.translation = main_camera.translation;
    base_transform.rotation = Quat::from_rotation_z(main_camera.rotation.x);

    let mut stick_transform = transform_query.get_mut(stick_entity).unwrap();
    stick_transform.translation = Vec3::ZERO.with_z(main_camera.depth);

    let mut pivot_transform = transform_query.get_mut(pivot_entity).unwrap();
    pivot_transform.rotation = Quat::from_rotation_x(main_camera.rotation.y);

    let mut camera_projection = projection_query.get_mut(camera_entity).unwrap();
    if let Projection::Perspective(ref mut projection) = *camera_projection {
        projection.fov = main_camera.fov;
    };
}

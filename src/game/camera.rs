//responsibilities:
//initializing useful camera abstraction, and associated camera settings
//exposing camera, and settings, as a resource
//allowing easy reset
//utilizing input plugin to enable movement, rotation, zoom, etc..

use crate::game::*;
use bevy::{prelude::*, render::view::RenderLayers};
use std::f32::consts::PI;

//========PLUGIN=========

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_resources, init).chain().after(os::init));
        app.add_systems(Update, (update_camera, sync_camera, sync_window));
    }
}

//========TYPES=========

#[derive(Resource)]
struct CameraSettings {
    //spatial
    translation: Vec3,
    rotation: Vec2,
    depth: f32,

    //projection
    fov: f32,
    near: f32,
    far: f32,

    //scaling for (continuously) changable values
    translation_speed: f32,
    rotation_speed: f32,
    depth_speed: f32,
    fov_speed: f32,
}
impl Default for CameraSettings {
    fn default() -> Self {
        let fov = PI / 4.;
        Self {
            //spatial
            translation: Vec3::new(0., -150., 0.),
            rotation: Vec2::new(0., PI / 8.),
            depth: 1000. / (fov / 2.).tan(),

            //projection
            fov,
            near: 0.,
            far: 2000.,

            //scaling for (continuously) changable values
            translation_speed: 800.,
            rotation_speed: 0.15,
            depth_speed: 300.,
            fov_speed: 0.1,
        }
    }
}

#[derive(Component)]
struct GameCameraMarker;

//========INIT=========

fn init_resources(mut commands: Commands) {
    commands.init_resource::<CameraSettings>()
}

fn init(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    main_window: Res<os::MainWindow>,
) {
    commands
        .spawn((TransformBundle::default(), GameCameraMarker))
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
                                        near: camera_settings.near,
                                        far: camera_settings.far,
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

//========UPDATE=========

fn update_camera(
    keyboard_axis: Res<input::KeyboardAxis>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mouse_axis: Res<input::MouseAxis>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    wheel_axis: Res<input::WheelAxis>,
    mut camera_settings: ResMut<CameraSettings>,
) {
    if mouse_buttons.pressed(MouseButton::Right) {
        //rotation
        camera_settings.rotation =
            (camera_settings.rotation - mouse_axis.0 * camera_settings.rotation_speed) % (2. * PI);
    } else if mouse_buttons.pressed(MouseButton::Middle) {
        //depth
        camera_settings.depth += mouse_axis.0.y * camera_settings.depth_speed;
        camera_settings.depth = camera_settings.depth.max(0.);
    }

    //translation
    let yaw = camera_settings.rotation.x;
    let speed = camera_settings.translation_speed;
    camera_settings.translation += Quat::from_rotation_z(yaw).mul_vec3(keyboard_axis.0 * speed);

    //fov
    camera_settings.fov -= wheel_axis.0.y * camera_settings.fov_speed;

    //reset
    if keyboard_buttons.pressed(KeyCode::KeyR) {
        *camera_settings = CameraSettings::default();
    }
}

fn sync_camera(
    camera_settings: Res<CameraSettings>,
    base_query: Query<Entity, With<GameCameraMarker>>,
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

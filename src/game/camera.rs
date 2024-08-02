use crate::game::{
    input::{KeyboardAxis, MouseAxis, WheelAxis},
    os::WindowSettings,
};
use bevy::{
    prelude::*,
    render::{
        camera::{OrthographicProjection, ScalingMode},
        view::RenderLayers,
    },
};
use std::f32::consts::PI;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (
                update_camera,
                update_projection,
                sync_camera,
                sync_projection,
            ),
        );
    }
}

#[derive(Component)]
struct GameCameraSingleton;

#[derive(Resource)]
struct GameCamera {
    translation: Vec3,
    rotation: Vec2,
    zoom: f32,
}
impl Default for GameCamera {
    fn default() -> Self {
        Self {
            translation: Vec3::new(0., 0., 0.),
            rotation: Vec2::new(0., 0.),
            zoom: 1.0,
        }
    }
}

#[derive(Resource)]
struct ProjectionSettings {
    orthographic: bool,
    fov: f32,
    near: f32,
    far: f32,
}
impl Default for ProjectionSettings {
    fn default() -> Self {
        Self {
            orthographic: true,
            fov: PI / 4.,
            near: 0.,
            far: 10000.,
        }
    }
}

#[derive(Resource)]
struct CameraSettings {
    move_speed: f32,
    turn_speed: f32,
    zoom_speed: f32,
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            move_speed: 800.,
            turn_speed: 0.15,
            zoom_speed: 1.5,
        }
    }
}

fn init(mut commands: Commands) {
    //, window_settings: Res<WindowSettings>
    let game_camera = GameCamera::default();
    let projection_settings = ProjectionSettings::default();
    let fov = ((projection_settings.fov / 2.).tan() * game_camera.zoom).atan() * 2.;
    commands
        .spawn((
            TransformBundle::from_transform(
                Transform::from_translation(game_camera.translation)
                    .with_rotation(Quat::from_rotation_z(game_camera.rotation.x)),
            ),
            GameCameraSingleton,
        ))
        .with_children(|builder| {
            builder
                .spawn(TransformBundle::from_transform(Transform::from_rotation(
                    Quat::from_rotation_x(game_camera.rotation.y),
                )))
                .with_children(|builder| {
                    builder
                        .spawn(TransformBundle::from_transform(
                            Transform::from_translation(Vec3::new(
                                0.,
                                0.,
                                1000. / (fov / 2.).tan(),
                            )),
                        ))
                        .with_children(|builder| {
                            builder.spawn((
                                Camera3dBundle {
                                    camera: Camera {
                                        clear_color: ClearColorConfig::Custom(Color::BLACK),
                                        order: 0,
                                        ..default()
                                    },
                                    projection: if projection_settings.orthographic {
                                        Projection::Orthographic(OrthographicProjection {
                                            near: projection_settings.near,
                                            far: projection_settings.far,
                                            scaling_mode: ScalingMode::AutoMin {
                                                min_width: 2000.,
                                                min_height: 2000.,
                                            },
                                            ..default()
                                        })
                                    } else {
                                        Projection::Perspective(PerspectiveProjection {
                                            near: projection_settings.near,
                                            far: projection_settings.far,
                                            fov,
                                            aspect_ratio: 1920. / 1080., //TODO: window_settings.aspect_ratio(),
                                        })
                                    },
                                    ..default()
                                },
                                RenderLayers::layer(0),
                            ));
                        });
                });
        });

    commands.insert_resource(game_camera);
    commands.insert_resource(projection_settings);
    commands.init_resource::<CameraSettings>();
}

fn update_camera(
    keyboard_axis: Res<KeyboardAxis>,
    mouse_axis: Res<MouseAxis>,
    wheel_axis: Res<WheelAxis>,
    camera_settings: Res<CameraSettings>,
    mut game_camera: ResMut<GameCamera>,
) {
    game_camera.translation += keyboard_axis.0 * camera_settings.move_speed;
    game_camera.rotation =
        (game_camera.rotation + mouse_axis.0 * camera_settings.turn_speed) % (2. * PI);
    game_camera.zoom += wheel_axis.0.y * camera_settings.zoom_speed;
}

fn sync_camera(
    game_camera: Res<GameCamera>,
    mut query: Query<&mut Children, With<GameCameraSingleton>>,
) {
    let child = query.get_single_mut().unwrap().iter().next().unwrap();
}

fn update_projection(
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut game_camera: ResMut<GameCamera>,
    mut projection_settings: ResMut<ProjectionSettings>,
) {
    if keyboard_buttons.just_pressed(KeyCode::KeyQ) {
        projection_settings.orthographic = !projection_settings.orthographic;
    }
    if keyboard_buttons.just_pressed(KeyCode::KeyR) {
        *game_camera = GameCamera::default();
        *projection_settings = ProjectionSettings::default();
    }
}

fn sync_projection() {}

// fn init(mut commands: Commands) {
//     commands
//         .spawn(GameCameraBundle::default())
//         .with_children(|builder| {
//             builder.spawn((
//                 Camera3dBundle {
//                     camera: Camera {
//                         clear_color: ClearColorConfig::Custom(Color::BLACK),
//                         order: 0,
//                         ..default()
//                     },
//                     transform: Transform::from_xyz(0., 0., projective_plane_distance()),
//                     ..default()
//                 },
//                 RenderLayers::layer(0),
//             ));
//             builder.spawn((
//                 Camera2dBundle {
//                     camera: Camera {
//                         clear_color: ClearColorConfig::None,
//                         order: 1,
//                         ..default()
//                     },
//                     transform: Transform::from_xyz(0., 0., projective_plane_distance()),
//                     ..default()
//                 },
//                 RenderLayers::layer(1),
//             ));
//         });
// }

// fn init_camera_reset(
//     mut main_camera_query: Query<&mut Projection, With<MainCamera>>,
//     mut overlay_camera_query: Query<&mut OrthographicProjection, With<OverlayCamera>>,
//     mut orbit_query: Query<(&mut Transform, &mut Fov, &mut Orthographic), With<Orbit>>,
//     window_settings: Res<WindowSettings>,
// ) {
//     let mut main_projection = main_camera_query.single_mut();
//     let mut overlay_projection = overlay_camera_query.single_mut();
//     let (mut transform, mut fov, mut orthographic) = orbit_query.single_mut();
//     reset_camera(
//         &mut transform,
//         &mut fov,
//         &mut orthographic,
//         &mut main_projection,
//         &mut overlay_projection,
//         window_settings.size.y,
//     );
// }

// fn update(
//     mut main_camera_query: Query<&mut Projection, With<MainCamera>>,
//     mut overlay_camera_query: Query<&mut OrthographicProjection, With<OverlayCamera>>,
//     mut orbit_query: Query<(&mut Transform, &mut Fov, &mut Orthographic), With<Orbit>>,
//     window_settings: Res<WindowSettings>,
//     mut keyboard_axis: ResMut<KeyboardAxis>,
//     mut mouse_axis: ResMut<MouseAxis>,
//     mut wheel_axis: ResMut<WheelAxis>,
// ) {
//     let mut main_projection = main_camera_query.single_mut();
//     let mut overlay_projection = overlay_camera_query.single_mut();
//     let (mut transform, mut fov, mut orthographic) = orbit_query.single_mut();

//     //reset position, orientation, zoom, projection mode
//     if keyboard_buttons.just_pressed(KeyCode::KeyR) {
//         reset_camera(
//             &mut transform,
//             &mut fov,
//             &mut orthographic,
//             &mut main_projection,
//             &mut overlay_projection,
//             window_settings.size.y,
//         );
//     }

//     //toggle projection mode
//     if keyboard_buttons.just_pressed(KeyCode::KeyQ) {
//         orthographic.0 = !orthographic.0;
//         *main_projection = make_projection(*fov, CAMERA_FAR, *orthographic, window_settings.size.y);
//     }

//     //pan
//     let mut direction: Vec3 = Vec3::ZERO;
//     if keyboard_buttons.pressed(KeyCode::KeyA) {
//         direction.x -= 1.;
//     }
//     if keyboard_buttons.pressed(KeyCode::KeyD) {
//         direction.x += 1.;
//     }
//     if keyboard_buttons.pressed(KeyCode::KeyW) {
//         direction.y += 1.;
//     }
//     if keyboard_buttons.pressed(KeyCode::KeyS) {
//         direction.y -= 1.;
//     }
//     if keyboard_buttons.pressed(KeyCode::Space) {
//         direction.z += 1.;
//     }
//     if keyboard_buttons.pressed(KeyCode::ControlLeft) {
//         direction.z -= 1.;
//     }
//     direction = direction.normalize_or_zero();
//     direction *= CAMERA_SPEED * time.delta_seconds();
//     let direction_xy = transform
//         .local_x()
//         .as_vec3()
//         .truncate()
//         .rotate(direction.truncate())
//         .extend(0.);
//     transform.translation += direction_xy;
//     transform.translation.z += direction.z;

//     //zoom
//     for scroll_event in mouse_wheel.read() {
//         if scroll_event.unit == MouseScrollUnit::Line {
//             if scroll_event.y > 0. {
//                 fov.0 *= CAMERA_FOV_SCALE;
//             } else if scroll_event.y < 0. {
//                 fov.0 /= CAMERA_FOV_SCALE;
//             }
//             match *main_projection {
//                 Projection::Perspective(ref mut projection) => {
//                     projection.fov = fov.0;
//                 }
//                 Projection::Orthographic(ref mut projection) => {
//                     projection.scaling_mode = ScalingMode::WindowSize(orthographic_window_scale(
//                         *fov,
//                         window_settings.size.y,
//                     ))
//                 }
//             }
//             overlay_projection.scaling_mode =
//                 ScalingMode::WindowSize(orthographic_window_scale(*fov, window_settings.size.y));
//         }
//         //pinch zoom unsupported because mobas use mice
//     }

//     //rotate
//     if mouse_buttons.pressed(MouseButton::Middle) {
//         let mut rot: Vec2 = Vec2::ZERO;
//         for motion in mouse_motion.read() {
//             rot -= motion.delta;
//         }
//         rot *= CAMERA_TURN_SPEED * time.delta_seconds();
//         transform.rotate_z(rot.x);
//         transform.rotate_local_x(rot.y);
//         //clamp x rotation
//         let mut angles = transform.rotation.to_euler(EulerRot::ZYX);
//         angles.2 -= 0.01;
//         if angles.2 < -PI / 2. {
//             angles.2 = PI;
//             transform.rotation = Quat::from_euler(EulerRot::ZYX, angles.0, angles.1, angles.2);
//         } else if angles.2 < 0. {
//             angles.2 = 0.;
//             transform.rotation = Quat::from_euler(EulerRot::ZYX, angles.0, angles.1, angles.2);
//         }
//     }
// }

// fn reset_camera(
//     trans: &mut Transform,
//     perspective: &mut PerspectiveProjection,
//     orthographic: &mut Orthographic,
//     projection: &mut Projection,
//     overlay_projection: &mut OrthographicProjection,
//     window_size: Vec2,
// ) {
//     *trans = Transform::IDENTITY;
//     *fov = CAMERA_FOV;
//     *orthographic = CAMERA_ORTHOGRAPHIC;
//     *projection = make_projection(*fov, CAMERA_FAR, *orthographic, window_size);
//     if let Projection::Orthographic(proj) = make_projection(*fov, CAMERA_FAR, true, window_size) {
//         *overlay_projection = proj;
//     }
// }

// fn make_projection(
//     perspective: PerspectiveProjection,
//     orthographic: bool,
//     window_size: Vec2,
// ) -> Projection {
//     if orthographic {
//         Projection::from(OrthographicProjection {
//             scaling_mode: ScalingMode::WindowSize(orthographic_window_scale(
//                 perspective.fov,
//                 window_size,
//             )),
//             near: perspective.near,
//             far: perspective.far,
//             // scaling_mode: ScalingMode::AutoMin {
//             //     min_width: 2000.,
//             //     min_height: 2000.,
//             // },
//             // scaling_mode: ScalingMode::AutoMax {
//             //     min_width: 2000.,
//             //     min_height: 2000.,
//             // },
//             ..default()
//         })
//     } else {
//         Projection::from(perspective)
//     }
// }

// //z distance required to fit the entire 2000 unit width ground plane into the perspective projection
// fn projective_plane_distance() -> f32 {
//     1000. / (CAMERA_FOV / 2.).tan()
// }

// fn orthographic_window_scale(fov: f32, window_size: Vec2) -> f32 {
//     window_size.min_element() / (2000. * (fov / 2.).tan() / (CAMERA_FOV / 2.).tan())
// }

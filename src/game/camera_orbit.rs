use crate::game::consts::*;
use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
    render::{
        camera::{OrthographicProjection, ScalingMode},
        view::RenderLayers,
    },
};
use std::f32::consts::PI;

pub struct CameraOrbitPlugin;
impl Plugin for CameraOrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_camera, init_camera_reset).chain());
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component, Default, Copy, Clone, Debug)]
struct Fov(f32);

#[derive(Component, Default, Copy, Clone, Debug)]
struct Orthographic(bool);

#[derive(Component, Default)]
struct Orbit;
#[derive(Bundle, Default)]
struct OrbitBundle {
    spatial: SpatialBundle,
    fov: Fov,
    orthographic: Orthographic,
    orbit: Orbit,
}

#[derive(Component, Default)]
struct MainCamera;
#[derive(Component, Default)]
struct OverlayCamera;

fn init_camera(mut commands: Commands) {
    commands
        .spawn(OrbitBundle::default())
        .with_children(|builder| {
            builder.spawn((
                Camera3dBundle {
                    camera: Camera {
                        clear_color: ClearColorConfig::Custom(Color::BLACK),
                        order: 0,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., projective_plane_distance()),
                    ..default()
                },
                RenderLayers::layer(0),
                MainCamera,
            ));
            builder.spawn((
                Camera2dBundle {
                    camera: Camera {
                        clear_color: ClearColorConfig::None,
                        order: 1,
                        ..default()
                    },
                    transform: Transform::from_xyz(0., 0., projective_plane_distance()),
                    ..default()
                },
                RenderLayers::layer(1),
                OverlayCamera,
            ));
        });
}

fn init_camera_reset(
    mut main_camera_query: Query<&mut Projection, With<MainCamera>>,
    mut overlay_camera_query: Query<&mut OrthographicProjection, With<OverlayCamera>>,
    mut orbit_query: Query<(&mut Transform, &mut Fov, &mut Orthographic), With<Orbit>>,
) {
    let mut main_projection = main_camera_query.single_mut();
    let mut overlay_projection = overlay_camera_query.single_mut();
    let (mut transform, mut fov, mut orthographic) = orbit_query.single_mut();
    reset_camera(
        &mut transform,
        &mut fov,
        &mut orthographic,
        &mut main_projection,
        &mut overlay_projection,
    );
}

fn update_camera(
    mut main_camera_query: Query<&mut Projection, With<MainCamera>>,
    mut overlay_camera_query: Query<&mut OrthographicProjection, With<OverlayCamera>>,
    mut orbit_query: Query<(&mut Transform, &mut Fov, &mut Orthographic), With<Orbit>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut main_projection = main_camera_query.single_mut();
    let mut overlay_projection = overlay_camera_query.single_mut();
    let (mut transform, mut fov, mut orthographic) = orbit_query.single_mut();

    //reset position, orientation, zoom, projection mode
    if keyboard_buttons.just_pressed(KeyCode::KeyR) {
        reset_camera(
            &mut transform,
            &mut fov,
            &mut orthographic,
            &mut main_projection,
            &mut overlay_projection,
        );
    }

    //toggle projection mode
    if keyboard_buttons.just_pressed(KeyCode::KeyQ) {
        orthographic.0 = !orthographic.0;
        *main_projection = make_projection(*fov, CAMERA_FAR, *orthographic);
    }

    //pan
    let mut direction: Vec3 = Vec3::ZERO;
    if keyboard_buttons.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keyboard_buttons.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keyboard_buttons.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keyboard_buttons.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if keyboard_buttons.pressed(KeyCode::Space) {
        direction.z += 1.;
    }
    if keyboard_buttons.pressed(KeyCode::ControlLeft) {
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
    for scroll_event in mouse_wheel.read() {
        if scroll_event.unit == MouseScrollUnit::Line {
            if scroll_event.y > 0. {
                fov.0 *= CAMERA_FOV_SCALE;
            } else if scroll_event.y < 0. {
                fov.0 /= CAMERA_FOV_SCALE;
            }
            match *main_projection {
                Projection::Perspective(ref mut projection) => {
                    projection.fov = fov.0;
                }
                Projection::Orthographic(ref mut projection) => {
                    projection.scaling_mode =
                        ScalingMode::WindowSize(orthographic_window_scale(*fov))
                }
            }
            overlay_projection.scaling_mode =
                ScalingMode::WindowSize(orthographic_window_scale(*fov));
        }
        //pinch zoom unsupported because mobas use mice
    }

    //rotate
    if mouse_buttons.pressed(MouseButton::Middle) {
        let mut rot: Vec2 = Vec2::ZERO;
        for motion in mouse_motion.read() {
            rot -= motion.delta;
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
    }
}

fn reset_camera(
    trans: &mut Transform,
    fov: &mut Fov,
    orthographic: &mut Orthographic,
    projection: &mut Projection,
    overlay_projection: &mut OrthographicProjection,
) {
    *trans = Transform::IDENTITY;
    *fov = Fov(CAMERA_FOV);
    *orthographic = Orthographic(CAMERA_ORTHOGRAPHIC);
    *projection = make_projection(*fov, CAMERA_FAR, *orthographic);
    if let Projection::Orthographic(proj) = make_projection(*fov, CAMERA_FAR, Orthographic(true)) {
        *overlay_projection = proj;
    }
}

fn make_projection(fov: Fov, far: f32, orthographic: Orthographic) -> Projection {
    if orthographic.0 {
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize(orthographic_window_scale(fov)),
            far,
            ..default()
        })
    } else {
        Projection::from(PerspectiveProjection {
            fov: fov.0,
            far,
            ..default()
        })
    }
}

fn projective_plane_distance() -> f32 {
    1000. / (CAMERA_FOV / 2.).tan()
}

fn orthographic_window_scale(fov: Fov) -> f32 {
    WINDOW_SIZE.y / (2000. * (fov.0 / 2.).tan() / (CAMERA_FOV / 2.).tan())
}

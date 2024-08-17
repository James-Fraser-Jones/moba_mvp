//responsibilities:
//collecting inputs into more useful abstractions
//(e.g. keyboardaxis, as a resource, of Vec3)
//facilitating different choices of concrete key bindings

use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

use crate::game::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyboardAxis>();
        app.init_resource::<MouseAxis>();
        app.init_resource::<WheelAxis>();
        app.init_resource::<ScreenAxis>();
        app.init_resource::<Cursor2D>();
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (
                get_keyboard_axis,
                get_mouse_axis,
                get_wheel_axis,
                get_cursor2d,
                get_screen_axis.after(get_cursor2d),
            )
                .in_set(UpdateInput),
        );
    }
}

const LINE_TO_PIXEL_SCALE: f32 = 50.;
const SCREEN_AXIS_RADIUS: f32 = 0.8;

#[derive(Resource, Default)]
pub struct KeyboardAxis(pub Vec3);

#[derive(Resource, Default)]
pub struct MouseAxis(pub Vec2);

#[derive(Resource, Default)]
pub struct WheelAxis(pub Vec2);

#[derive(Resource, Default)]
pub struct Cursor2D(pub Vec2);

#[derive(Resource, Default)]
pub struct ScreenAxis(pub Vec2);

fn init(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut last_cursor_position: ResMut<Cursor2D>,
) {
    let window = window_query.single();
    last_cursor_position.0 = window.size() / 2.;
}

fn get_keyboard_axis(
    time: Res<Time>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut keyboard_axis: ResMut<KeyboardAxis>,
) {
    let mut axis: Vec3 = Vec3::ZERO;
    if keyboard_buttons.pressed(KeyCode::KeyW) {
        axis.y += 1.;
    }
    if keyboard_buttons.pressed(KeyCode::KeyS) {
        axis.y -= 1.;
    }
    if keyboard_buttons.pressed(KeyCode::KeyD) {
        axis.x += 1.;
    }
    if keyboard_buttons.pressed(KeyCode::KeyA) {
        axis.x -= 1.;
    }
    if keyboard_buttons.pressed(KeyCode::Space) {
        axis.z += 1.;
    }
    if keyboard_buttons.pressed(KeyCode::ControlLeft) {
        axis.z -= 1.;
    }
    axis = axis.clamp_length_max(1.) * time.delta_seconds();
    keyboard_axis.0 = axis;
}

fn get_mouse_axis(
    time: Res<Time>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_axis: ResMut<MouseAxis>,
) {
    let mut axis: Vec2 = Vec2::ZERO;
    for motion in mouse_motion.read() {
        axis += motion.delta;
    }
    axis *= time.delta_seconds();
    mouse_axis.0 = axis;
}

fn get_wheel_axis(
    time: Res<Time>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut wheel_axis: ResMut<WheelAxis>,
) {
    let mut axis: Vec2 = Vec2::ZERO;
    for motion in mouse_wheel.read() {
        match motion.unit {
            MouseScrollUnit::Pixel => {
                axis += Vec2::new(motion.x, motion.y);
            }
            MouseScrollUnit::Line => {
                axis += Vec2::new(motion.x, motion.y) * LINE_TO_PIXEL_SCALE;
            }
        }
    }
    axis *= time.delta_seconds();
    wheel_axis.0 = axis;
}

fn get_cursor2d(
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut last_cursor_position: ResMut<Cursor2D>,
) {
    let window = window_query.single();
    if let Some(cursor_position) = window.cursor_position() {
        last_cursor_position.0 = cursor_position;
    }
}

fn get_screen_axis(
    time: Res<Time>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut screen_axis: ResMut<ScreenAxis>,
    last_cursor_position: Res<Cursor2D>,
) {
    let window = window_query.single();
    let window_size = window.resolution.size(); //range ([0, WINDOW_WIDTH], [0, WINDOW_HEIGHT]), +y down
    let cursor_centered = last_cursor_position.0 - window_size / 2.; //range ([-WINDOW_WIDTH, WINDOW_WIDTH], [-WINDOW_HEIGHT, WINDOW_HEIGHT]), +y down
    let cursor_scaled = 2. * cursor_centered / window_size; //range ([-1., 1.], [-1., 1.]), +y down
    let cursor_length = (cursor_scaled.length().clamp(SCREEN_AXIS_RADIUS, 1.) - SCREEN_AXIS_RADIUS)
        / (1. - SCREEN_AXIS_RADIUS); //range [0, 1] from start of SCREEN_AXIS_RADIUS to edge of screen, in each axis, clamped at the corners
    let cursor_vector = cursor_centered.normalize() * cursor_length; // +y down
    let mut axis: Vec2 = Vec2::new(cursor_vector.x, -cursor_vector.y); // +y up
    axis *= time.delta_seconds();
    screen_axis.0 = axis;
}

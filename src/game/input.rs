//responsibilities:
//collecting inputs into more useful abstractions
//(e.g. keyboardaxis, as a resource, of Vec3)
//facilitating different choices of concrete key bindings

use bevy::{
    input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyboardAxis>();
        app.init_resource::<MouseAxis>();
        app.init_resource::<WheelAxis>();
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

const LINE_TO_PIXEL_SCALE: f32 = 50.;

#[derive(Resource, Default)]
pub struct KeyboardAxis(pub Vec3);

#[derive(Resource, Default)]
pub struct MouseAxis(pub Vec2);

#[derive(Resource, Default)]
pub struct WheelAxis(pub Vec2);

fn init() {}

fn update(
    time: Res<Time>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut keyboard_axis: ResMut<KeyboardAxis>,
    mut mouse_axis: ResMut<MouseAxis>,
    mut wheel_axis: ResMut<WheelAxis>,
) {
    //keyboard
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

    //mouse
    let mut axis: Vec2 = Vec2::ZERO;
    for motion in mouse_motion.read() {
        axis += motion.delta;
    }
    axis *= time.delta_seconds();
    mouse_axis.0 = axis;

    //mouse wheel
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

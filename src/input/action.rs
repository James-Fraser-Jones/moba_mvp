use crate::*;
use bevy::prelude::*;

pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>();
        app.add_systems(Startup, init.in_set(ActionSet).in_set(InputSet));
        app.add_systems(PreUpdate, update.in_set(ActionSet).in_set(InputSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionSet;

#[derive(Event)]
pub enum ActionEvent {
    Move(Vec2),
    AttackMove(Vec2),
    Stop,
}

fn init() {}

fn update(
    cursor_2d: Res<input::CursorPosition2D>,
    camera_query: Query<(&Camera, &GlobalTransform), With<OrbitDistance>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    mut action_events: EventWriter<ActionEvent>,
) {
    let (camera, transform) = camera_query.single();
    if let Some(point) = pixel_to_horizontal_plane(cursor_2d.0, 0., camera, &transform) {
        if mouse_buttons.just_pressed(MouseButton::Right) {
            action_events.send(ActionEvent::Move(point));
        }
        if keyboard_buttons.just_pressed(KeyCode::KeyA) {
            action_events.send(ActionEvent::AttackMove(point));
        }
    }
    if keyboard_buttons.just_pressed(KeyCode::KeyS) {
        action_events.send(ActionEvent::Stop);
    }
}

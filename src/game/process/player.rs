//responsibilities:
//attaching to, and keeping track of, player entities, via a tag
//utilizing input and camera plugins to drive player actions, through events sent to the logic plugin

use crate::game::*;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RightPressed>();
        app.add_systems(Startup, init.in_set(PlayerSet).in_set(ProcessSet));
        app.add_systems(FixedUpdate, update.in_set(PlayerSet).in_set(ProcessSet));
    }
}

const PLAYER_ID: i32 = 0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Resource, Default)]
pub struct RightPressed(pub bool);

#[derive(Resource)]
pub struct Player(pub Entity);

fn init(mut commands: Commands, query: Query<(Entity, &PlayerID)>) {
    let (player, _) = query
        .iter()
        .skip_while(|(_, player_id)| player_id.0 != PLAYER_ID)
        .next()
        .unwrap();
    commands.insert_resource(Player(player));
}

fn update(
    cursor_2d: Res<input::CursorPosition2D>,
    camera_query: Query<(&Camera, &GlobalTransform), With<OrbitDistance>>,
    mut mouse_events: EventReader<bevy::input::mouse::MouseButtonInput>,
    mut keyboard_events: EventReader<bevy::input::keyboard::KeyboardInput>,
    player: Res<Player>,
    mut player_query: Query<&mut MovePosition>,
    mut right_pressed: ResMut<RightPressed>,
) {
    let (camera, transform) = camera_query.single();
    let point = pixel_to_horizontal_plane(cursor_2d.0, 0., camera, &transform);
    let mut move_position = player_query.get_mut(player.0).unwrap();

    println!("UPDATE");

    //move
    //"pressed" can also be missed if button pressed briefly during frames that did not run FixedUpdate
    for event in mouse_events.read() {
        match event.state {
            bevy::input::ButtonState::Pressed => match event.button {
                MouseButton::Right => {
                    println!("PRESSED");
                    right_pressed.0 = true;
                }
                _ => {}
            },
            bevy::input::ButtonState::Released => match event.button {
                MouseButton::Right => {
                    println!("RELEASED");
                    right_pressed.0 = false;
                }
                _ => {}
            },
        }
    }
    if right_pressed.0 {
        move_position.0 = point;
    }

    //required as "just_pressed" methods are not safe within FixedUpdate
    //(i.e. can be arbitrarily missed or duplicated)
    for event in keyboard_events.read() {
        match event.state {
            bevy::input::ButtonState::Pressed => match event.key_code {
                KeyCode::KeyA => {
                    //attack move
                    move_position.0 = point;
                }
                KeyCode::KeyS => {
                    //stop move
                    move_position.0 = None;
                }
                _ => {}
            },
            bevy::input::ButtonState::Released => {}
        }
    }
}

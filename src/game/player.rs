//responsibilities:
//attaching to, and keeping track of, player entities, via a tag
//utilizing input and camera plugins to drive player actions, through events sent to the logic plugin

use crate::game::*;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.after(spawn::init));
        app.add_systems(Update, update.in_set(UpdatePlayer));
    }
}

const PLAYER_ID: i32 = 0;

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
    cursor_2d: Res<input::Cursor2D>,
    camera_query: Query<(&Camera, &GlobalTransform), With<OrbitDistance>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    player: Res<Player>,
    mut player_query: Query<&mut MovePosition>,
) {
    let (camera, transform) = camera_query.single();
    let point = pixel_to_horizontal_plane(cursor_2d.0, 0., camera, &transform);
    let mut move_position = player_query.get_mut(player.0).unwrap();

    //move
    if mouse_buttons.pressed(MouseButton::Right) {
        move_position.0 = point;
    }
    //attack move
    if keyboard_buttons.just_pressed(KeyCode::KeyA) {
        move_position.0 = point;
    }
    //stop move
    if keyboard_buttons.just_pressed(KeyCode::KeyS) {
        move_position.0 = None;
    }
}

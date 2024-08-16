//responsibilities:
//attaching to, and keeping track of, player entities, via a tag
//utilizing input and camera plugins to drive player actions, through events sent to the logic plugin

use super::{types::*, *};
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor3D>();
        app.add_systems(Startup, init.after(logic::spawn::init));
        app.add_systems(Update, update.in_set(UpdatePlayer));
    }
}

const PLAYER_ID: i32 = 0;

#[derive(Resource)]
pub struct Player(pub Entity);

#[derive(Resource, Default)]
pub struct Cursor3D(pub Option<Vec2>);

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
    mut cursor_3d: ResMut<Cursor3D>,
    mut camera_query: Query<
        (&Camera, &GlobalTransform),
        With<cameras::orbit_camera::OrbitDistance>,
    >,

    mouse_buttons: Res<ButtonInput<MouseButton>>,
    keyboard_buttons: Res<ButtonInput<KeyCode>>,
    player: Res<Player>,
    mut player_query: Query<&mut MovePosition>,
) {
    //update cursor3d
    let (camera, transform) = camera_query.single_mut();
    let ground_plane_z = 0.;
    cursor_3d.0 = cameras::orbit_camera::pixel_to_horizontal_plane(
        cursor_2d.0,
        ground_plane_z,
        camera,
        &transform,
    );

    let mut move_position = player_query.get_mut(player.0).unwrap();
    //move
    if mouse_buttons.pressed(MouseButton::Right) {
        if let Some(point) = cursor_3d.0 {
            move_position.0 = Some(point);
        }
    }
    //attack move
    if keyboard_buttons.just_pressed(KeyCode::KeyA) {
        if let Some(point) = cursor_3d.0 {
            move_position.0 = Some(point);
        }
    }
    //stop move
    if keyboard_buttons.just_pressed(KeyCode::KeyS) {
        move_position.0 = None;
    }
}

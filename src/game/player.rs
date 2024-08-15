//responsibilities:
//attaching to, and keeping track of, player entities, via a tag
//utilizing input and camera plugins to drive player actions, through events sent to the logic plugin

use super::{logic::types::*, *};
use bevy::prelude::*;

const PLAYER_ID: i32 = 0;

#[derive(Resource)]
pub struct Player(pub Entity);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.after(logic::init));
        app.add_systems(Update, update);
    }
}

fn init(mut commands: Commands, query: Query<(Entity, &PlayerID)>) {
    let (player, _) = query
        .iter()
        .skip_while(|(_, player_id)| player_id.0 != PLAYER_ID)
        .next()
        .unwrap();
    commands.insert_resource(Player(player));
}

fn update(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut MovePosition>,
    player: Res<Player>,
    cursor_world_position: Res<input::CursorWorldPosition>,
) {
    if mouse_buttons.just_pressed(MouseButton::Right) {
        if let Some(point) = cursor_world_position.0 {
            query.get_mut(player.0).unwrap().0 = Some(point);
        }
    }
}

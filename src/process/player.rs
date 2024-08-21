use crate::*;
use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerSet));
        app.add_systems(FixedUpdate, update.in_set(PlayerSet));
    }
}

const PLAYER_HANDLE: InputHandle = InputHandle::Input0;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Resource)]
pub struct Player(pub Entity);

//TODO: move to spawn system
fn init(mut commands: Commands, query: Query<(Entity, &InputHandle)>) {
    let (player_entity, _) = query
        .iter()
        .skip_while(|(_, player_handle)| **player_handle != PLAYER_HANDLE)
        .next()
        .unwrap();
    commands.insert_resource(Player(player_entity));
}

fn update(
    player: Res<Player>,
    mut player_query: Query<&mut MovePosition>,
    mut action_events: EventReader<ActionEvent>,
) {
    let mut move_position = player_query.get_mut(player.0).unwrap();
    for event in action_events.read() {
        match *event {
            ActionEvent::Move(point) => {
                move_position.0 = Some(point);
            }
            ActionEvent::AttackMove(point) => {
                move_position.0 = Some(point);
            }
            ActionEvent::Stop => {
                move_position.0 = None;
            }
        }
    }
}

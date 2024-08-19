use crate::game::*;
use bevy::prelude::*;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            init.in_set(PhysicsSet)
                .in_set(DeterministicSet)
                .in_set(ProcessSet),
        );
        app.add_systems(
            FixedUpdate,
            update
                .in_set(PhysicsSet)
                .in_set(DeterministicSet)
                .in_set(ProcessSet),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

fn init() {}

fn update() {}

use crate::*;
use bevy::prelude::*;

pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(ActionSet).in_set(InputSet));
        app.add_systems(PreUpdate, update.in_set(ActionSet).in_set(InputSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionSet;

fn init() {}

fn update() {}

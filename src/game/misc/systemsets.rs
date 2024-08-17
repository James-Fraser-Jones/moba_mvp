//use crate::game::*;
use bevy::prelude::*;

pub struct SetsPlugin;
impl Plugin for SetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PreUpdate, InputSet.after(bevy::input::InputSystem));
        app.configure_sets(
            FixedUpdate,
            (
                (PlayerSet, NetworkingInSet),
                LogicSet,
                PhysicsSet,
                NetworkingOutSet,
            )
                .chain(),
        );
        app.configure_sets(
            PostUpdate,
            ProjectCameraSet.after(TransformSystem::TransformPropagate),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkingInSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NetworkingOutSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectCameraSet;

//use crate::game::*;
use bevy::prelude::*;

pub struct SetsPlugin;
impl Plugin for SetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                UpdateInput,
                UpdateCameras::PreLogic,
                UpdatePlayer,
                UpdateLogic,
                UpdateCameras::PostLogic,
                UpdateGraphics,
            )
                .chain(),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateInput;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCameras {
    PreLogic,
    PostLogic,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdatePlayer;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateLogic;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateGraphics;

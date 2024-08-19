mod misc;
use misc::*;
mod gametypes;
use gametypes::*;
mod input;
use input::*;
mod output;
use output::*;
mod process;
use process::*;
mod order;
use order::*;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GizmosPlugin)
            .add(HealthbarPlugin)
            .add(AxisPlugin)
            .add(LogicPlugin)
            .add(MapPlugin)
            .add(ModelPlugin)
            .add(OrbitCameraPlugin)
            .add(OSPlugin)
            .add(OverlayCameraPlugin)
            .add(PlayerPlugin)
            .add(OrderPlugin)
    }
}

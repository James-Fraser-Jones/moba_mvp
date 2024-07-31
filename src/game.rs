mod camera_orbit;
// mod camera_orthographic;
// mod camera_perspective;
pub mod consts;
mod graphics;
mod logic;
pub mod quit;
mod types;
mod utils;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera_orbit::*;
// use camera_orthographic::*;
// use camera_perspective::*;
use graphics::*;
use logic::*;
use quit::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogicPlugin)
            .add(GraphicsPlugin)
            .add(CameraOrbitPlugin)
            .add(QuitPlugin)
    }
}

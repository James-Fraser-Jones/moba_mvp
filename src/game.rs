mod camera;
pub mod consts;
mod graphics;
mod logic;
mod quit;
mod threedee;
mod types;
mod utils;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::*;
use graphics::*;
use logic::*;
use quit::*;
use threedee::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogicPlugin)
            .add(GraphicsPlugin)
            .add(CameraPlugin)
            .add(QuitPlugin)
        //.add(ThreeDeePlugin)
    }
}

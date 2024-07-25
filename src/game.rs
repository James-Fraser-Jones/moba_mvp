mod camera;
mod graphics;
mod logic;
mod quit;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::*;
use graphics::*;
use logic::*;
use quit::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LogicPlugin)
            .add(GraphicsPlugin)
            .add(CameraPlugin)
            .add(QuitPlugin)
    }
}

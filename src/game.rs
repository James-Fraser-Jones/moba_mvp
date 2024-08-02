mod consts;
mod types;
mod utils;

mod camera;
mod graphics;
mod logic;
mod os;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(os::OSPlugin)
            .add(logic::LogicPlugin)
            .add(graphics::GraphicsPlugin)
            .add(camera::CameraPlugin)
    }
}

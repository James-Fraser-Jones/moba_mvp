mod cameras;
mod dev;
mod graphics;
mod input;
mod logic;
mod os;
mod player;
mod types;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(os::OSPlugin)
            .add(input::InputPlugin)
            .add(logic::LogicPlugin)
            .add(graphics::GraphicsPlugin)
            .add(cameras::CamerasPlugin)
            .add(dev::DevPlugin)
            .add(player::PlayerPlugin)
    }
}

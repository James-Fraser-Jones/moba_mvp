mod camera;
mod graphics;
mod input;
mod os;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(os::OSPlugin)
            .add(input::InputPlugin)
            .add(graphics::GraphicsPlugin)
            .add(camera::CameraPlugin)
    }
}

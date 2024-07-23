mod camera;
mod init;
mod quit;
mod update;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera::*;
use init::*;
use quit::*;
use update::*;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(QuitPlugin)
            .add(CameraPlugin)
            .add(InitPlugin)
            .add(UpdatePlugin)
    }
}

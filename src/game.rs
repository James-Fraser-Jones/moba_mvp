pub mod misc;
pub use misc::*;
pub mod gametypes;
pub use gametypes::*;
pub mod input;
pub use input::*;
pub mod output;
pub use output::*;
pub mod process;
pub use process::*;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(GizmosPlugin)
            .add(HealthbarPlugin)
            .add(InputPlugin)
            .add(LogicPlugin)
            .add(MapPlugin)
            .add(ModelPlugin)
            .add(OrbitCameraPlugin)
            .add(OSPlugin)
            .add(OverlayCameraPlugin)
            .add(PlayerPlugin)
            .add(SpawnPlugin)
            .add(SetsPlugin)
    }
}

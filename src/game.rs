mod gametypes;
pub use gametypes::*;

mod systemsets;
pub use systemsets::*;
mod gizmos;
pub use gizmos::*;
mod healthbar;
pub use healthbar::*;
mod input;
pub use input::*;
mod logic;
pub use logic::*;
mod map;
pub use map::*;
mod model;
pub use model::*;
mod orbit_camera;
pub use orbit_camera::*;
mod os;
pub use os::*;
mod overlay_camera;
pub use overlay_camera::*;
mod player;
pub use player::*;
mod spawn;
pub use spawn::*;

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

mod cameras;
mod graphics;
mod input;
mod logic;
mod os;
mod player;
mod types;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamePlugins);
        app.configure_sets(
            Update,
            (
                UpdateInput,
                UpdateCameras::PreLogic,
                UpdatePlayer,
                UpdateLogic,
                UpdateCameras::PostLogic,
                UpdateGraphics,
            )
                .chain(),
        );
    }
}

struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(os::OSPlugin)
            .add(input::InputPlugin)
            .add_group(CameraPlugins)
            .add(player::PlayerPlugin)
            .add_group(LogicPlugins)
            .add_group(GraphicsPlugins)
    }
}

struct CameraPlugins;
impl PluginGroup for CameraPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(cameras::CamerasPlugin)
            .add(cameras::orbit_camera::OrbitCameraPlugin)
            .add(cameras::overlay_camera::OverlayCameraPlugin)
    }
}

struct GraphicsPlugins;
impl PluginGroup for GraphicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(graphics::GraphicsPlugin)
            .add(graphics::model::ModelPlugin)
            .add(graphics::map::MapPlugin)
            .add(graphics::healthbar::HealthbarPlugin)
            .add(graphics::gizmos::GizmosPlugin)
    }
}

struct LogicPlugins;
impl PluginGroup for LogicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(logic::LogicPlugin)
            .add(logic::spawn::SpawnPlugin)
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateInput;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateCameras {
    PreLogic,
    PostLogic,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdatePlayer;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateLogic;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateGraphics;

pub mod helpers;
mod plugins;

use bevy::{prelude::*, window::WindowMode};
use helpers::consts::*;
use plugins::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: APP_NAME.into(),
                    name: Some(APP_NAME.into()),
                    position: WindowPosition::At(IVec2::ZERO),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
            camera::CameraPlugin,
            quit::QuitPlugin,
            init::InitPlugin,
            update::UpdatePlugin,
        ))
        .run()
}

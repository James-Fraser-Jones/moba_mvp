mod game;

use bevy::{
    prelude::*,
    window::{Cursor, CursorGrabMode, MonitorSelection, WindowMode},
};
use consts::*;
use game::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: APP_NAME.into(),
                    name: Some(APP_NAME.into()),
                    mode: WindowMode::BorderlessFullscreen,
                    cursor: Cursor {
                        grab_mode: CursorGrabMode::Confined,
                        visible: false,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            GamePlugins,
        ))
        .run()
}

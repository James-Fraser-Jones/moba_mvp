mod examples;
mod game;
use bevy::{
    prelude::*,
    window::{Cursor, WindowResolution},
};
use game::{consts::*, GamePlugins};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: WINDOW_NAME.into(),
                    name: Some(WINDOW_NAME.into()),
                    position: WindowPosition::new(WINDOW_POSITION),
                    resolution: WindowResolution::new(WINDOW_SIZE.x, WINDOW_SIZE.y),
                    mode: WINDOW_MODE,
                    cursor: Cursor {
                        grab_mode: WINDOW_CURSOR_GRAB_MODE,
                        visible: true,
                        ..default()
                    },
                    ..default()
                }),
                ..default()
            }),
            GamePlugins,
            //examples::animation::ExampleAnimationPlugin,
            //examples::physics::ExamplePhysicsPlugin,
        ))
        .run()
}

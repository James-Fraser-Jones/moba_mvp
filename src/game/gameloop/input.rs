mod keyboard;
mod mouse;

use bevy::prelude::*;

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((keyboard::KeyboardPlugin, mouse::MousePlugin));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

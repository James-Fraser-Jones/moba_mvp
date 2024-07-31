mod exit;
mod window;

use bevy::prelude::*;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((window::WindowPlugin, exit::ExitPlugin));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

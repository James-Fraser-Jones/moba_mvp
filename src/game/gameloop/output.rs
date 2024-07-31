mod graphics;
mod sound;

use bevy::prelude::*;

pub struct OutputPlugin;
impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((graphics::GraphicsPlugin, sound::SoundPlugin));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

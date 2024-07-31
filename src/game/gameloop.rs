mod input;
mod logic;
mod output;

use bevy::prelude::*;

pub struct GameLoopPlugin;
impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((input::InputPlugin, logic::LogicPlugin, output::OutputPlugin));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

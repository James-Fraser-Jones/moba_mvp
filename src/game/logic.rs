//responsibilities:
//drive the main logic of the game
//no consideration of anything other than core logic
//necessarily must perform physics calculations since they dictate discrete logic such as whether a skillshot landed, etc..
//entirely self contained, no direct communication with any other plugins
//can recieve events from player plugins to determine player actions, through a strongly-typed interface (only source of non-determinism)

use bevy::prelude::*;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

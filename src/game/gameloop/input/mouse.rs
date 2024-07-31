use bevy::prelude::*;

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

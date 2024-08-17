//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

pub mod gizmos;
pub mod healthbar;
pub mod map;
pub mod model;

use super::UpdateGraphics;
use bevy::prelude::*;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update.in_set(UpdateGraphics));
    }
}

fn init() {}

fn update() {}

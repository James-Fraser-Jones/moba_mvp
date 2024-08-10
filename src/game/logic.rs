//responsibilities:
//drive the main logic of the game
//no consideration of anything other than core logic
//necessarily must perform physics calculations since they dictate discrete logic such as whether a skillshot landed, etc..
//entirely self contained, no direct communication with any other plugins
//can recieve events from player plugins to determine player actions, through a strongly-typed interface (only source of non-determinism)

use crate::game::*;
use bevy::color::palettes::css;
use bevy::prelude::*;

use super::graphics::{self, Display};

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

const LANE_WIDTH: f32 = 130.;
const OUTER_WALL_WIDTH: f32 = 100.;

#[derive(Component)]
struct Health(f32);
#[derive(Component)]
struct Radius(f32);

#[derive(Bundle)]
struct Minion {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    display: graphics::Display,
}
impl Default for Minion {
    fn default() -> Self {
        let radius = 50.;
        Self {
            spatial: SpatialBundle::default(),
            radius: Radius(radius),
            health: Health(100.),
            display: Display {
                allowed_mesh: graphics::AllowedMesh::Sphere(Sphere::new(radius)),
                color: Color::Srgba(css::ALICE_BLUE),
            },
        }
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Minion::default());
}

fn update() {}

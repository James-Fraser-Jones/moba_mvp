//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

pub mod healthbar;
pub mod map;
pub mod model;

use super::types::*;
use bevy::{color::palettes::css, prelude::*};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            model::ModelPlugin,
            healthbar::HealthbarPlugin,
            map::MapPlugin,
        ));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

const RED_TEAM_COLOR: Color = Color::Srgba(css::TOMATO);
const BLUE_TEAM_COLOR: Color = Color::Srgba(css::DEEP_SKY_BLUE);
const NO_TEAM_COLOR: Color = Color::Srgba(css::SEA_GREEN);
pub fn team_color(team: Option<Team>) -> Color {
    match team {
        Some(team) => match team {
            Team::Red => RED_TEAM_COLOR,
            Team::Blue => BLUE_TEAM_COLOR,
        },
        None => NO_TEAM_COLOR,
    }
}

//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

pub mod healthbar;
pub mod map;
pub mod model;

use super::{logic::types::*, *};
use bevy::{color::palettes::css, pbr::wireframe::Wireframe, prelude::*, render::*};
use cameras::orbit_camera::OrbitDistance;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;
use std::sync::LazyLock;

pub const RED_TEAM_COLOR: Color = Color::Srgba(css::TOMATO);
pub const BLUE_TEAM_COLOR: Color = Color::Srgba(css::DEEP_SKY_BLUE);
pub const NO_TEAM_COLOR: Color = Color::Srgba(css::SEA_GREEN);

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(model::ModelPlugin);
        app.add_systems(Startup, (init, map::init));
        app.add_systems(
            Update,
            (
                draw_cursor,
                map::update,
                healthbar::add_healthbars,
                healthbar::update_healthbars,
            ),
        );
    }
}

fn init() {}

fn draw_cursor(cursor_world_position: Res<input::CursorWorldPosition>, mut gizmos: Gizmos) {
    if let Some(point) = cursor_world_position.0 {
        gizmos.circle(
            point.extend(0.01),
            Dir3::new(Vec3::Z).unwrap(),
            10.,
            Color::WHITE,
        );
        gizmos.arrow(point.extend(30.), point.extend(0.01), Color::WHITE);
    }
}

pub fn team_color(team: Option<Team>) -> Color {
    match team {
        Some(team) => match team {
            Team::Red => RED_TEAM_COLOR,
            Team::Blue => BLUE_TEAM_COLOR,
        },
        None => NO_TEAM_COLOR,
    }
}

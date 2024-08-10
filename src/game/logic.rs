//responsibilities:
//drive the main logic of the game
//no consideration of anything other than core logic
//necessarily must perform physics calculations since they dictate discrete logic such as whether a skillshot landed, etc..
//entirely self contained, no direct communication with any other plugins
//can recieve events from player plugins to determine player actions, through a strongly-typed interface (only source of non-determinism)

use crate::game::*;
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

#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
pub enum Team {
    #[default]
    Red,
    Blue,
}

#[derive(Bundle)]
struct Spawner {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Spawner {
    fn new(pos: Vec2, team: Team) -> Self {
        let radius = 30.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health(500.),
            team,
            display: Display {
                allowed_mesh: graphics::AllowedMesh::Sphere(Sphere::new(radius)),
                color: graphics::team_color(team),
                wireframe: true,
            },
        }
    }
}

#[derive(Bundle)]
struct Tower {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Tower {
    fn new(pos: Vec2, team: Team) -> Self {
        let radius = 20.;
        let half_height = 30.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(half_height),
            )),
            radius: Radius(radius),
            health: Health(500.),
            team,
            display: Display {
                allowed_mesh: graphics::AllowedMesh::Cylinder(Cylinder::new(
                    radius,
                    half_height * 2.,
                )),
                color: graphics::team_color(team),
                wireframe: true,
            },
        }
    }
}

#[derive(Bundle)]
struct Minion {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Minion {
    fn new(pos: Vec2, team: Team) -> Self {
        let radius = 10.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(radius))),
            radius: Radius(radius),
            health: Health(100.),
            team,
            display: Display {
                allowed_mesh: graphics::AllowedMesh::Cuboid(Cuboid::from_length(radius)),
                color: graphics::team_color(team),
                wireframe: true,
            },
        }
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Tower::new(Vec2::new(-50., 50.), Team::Red));
    commands.spawn(Tower::new(Vec2::new(50., 50.), Team::Blue));
    commands.spawn(Spawner::new(Vec2::new(-50., 0.), Team::Red));
    commands.spawn(Spawner::new(Vec2::new(50., 0.), Team::Blue));
    commands.spawn(Minion::new(Vec2::new(-50., -50.), Team::Red));
    commands.spawn(Minion::new(Vec2::new(50., -50.), Team::Blue));
}

fn update() {}

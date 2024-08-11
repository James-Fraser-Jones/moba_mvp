//responsibilities:
//drive the main logic of the game
//no consideration of anything other than core logic
//necessarily must perform physics calculations since they dictate discrete logic such as whether a skillshot landed, etc..
//entirely self contained, no direct communication with any other plugins
//can recieve events from player plugins to determine player actions, through a strongly-typed interface (only source of non-determinism)

use crate::game::*;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::sync::LazyLock;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

const LANE_WIDTH: f32 = 130.;
const OUTER_WALL_WIDTH: f32 = 100.;
const NUM_ADVOCATES: i32 = 5;

static RED_TRANSFORM: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_translation(Vec2::splat(-1000.).extend(0.)));
static BLUE_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| {
    Transform::from_translation(Vec2::splat(1000.).extend(0.))
        .with_rotation(Quat::from_rotation_z(PI))
});
fn team_transform(team: Team) -> Transform {
    match team {
        Team::Red => *RED_TRANSFORM,
        Team::Blue => *BLUE_TRANSFORM,
    }
}

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

#[derive(Component, Default, PartialEq, Eq, Clone, Copy)]
pub enum Lane {
    Bot,
    #[default]
    Mid,
    Top,
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
        let radius = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health(1000.),
            team,
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Sphere(Sphere::new(radius)),
                color: graphics::team_color(team),
                wireframe: false,
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
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Cylinder(Cylinder::new(
                    radius,
                    half_height * 2.,
                )),
                color: graphics::team_color(team),
                wireframe: false,
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
        let radius = 8.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(radius))),
            radius: Radius(radius),
            health: Health(100.),
            team,
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Cuboid(Cuboid::from_length(radius * 2.)),
                color: graphics::team_color(team),
                wireframe: false,
            },
        }
    }
}

#[derive(Bundle)]
struct Advocate {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Advocate {
    fn new(pos: Vec2, team: Team) -> Self {
        let radius = 12.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(radius))),
            radius: Radius(radius),
            health: Health(200.),
            team,
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Sphere(Sphere::new(radius)),
                color: graphics::team_color(team),
                wireframe: false,
            },
        }
    }
}

#[derive(Bundle)]
struct Core {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Core {
    fn new(pos: Vec2, team: Team) -> Self {
        let radius = 35.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health(5000.),
            team,
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Sphere(Sphere::new(radius)),
                color: graphics::team_color(team),
                wireframe: false,
            },
        }
    }
}

fn init(mut commands: Commands) {
    spawn_towers(&mut commands);
}

fn spawn_towers(commands: &mut Commands) {
    let origin = Vec2::splat(OUTER_WALL_WIDTH + LANE_WIDTH / 2.).extend(0.);
    let tower_radius = 20.;
    let zig_spacing = LANE_WIDTH / 2. - tower_radius;
    for team in [Team::Red, Team::Blue] {
        for lane in [Lane::Top, Lane::Mid, Lane::Bot] {
            let ang = PI / 4. * lane as i32 as f32;
            let zig = lane != Lane::Bot;
            let points = if lane == Lane::Mid {
                vec![540., 780., 1000.]
            } else {
                vec![500., 830., 1400.]
            };
            for point in zig_zag(points, zig, zig_spacing) {
                let point = point.extend(0.);
                let ang_point = Transform::from_rotation(Quat::from_rotation_z(ang)) * point;
                let origin_point = ang_point + origin;
                let team_point = team_transform(team) * origin_point;
                commands.spawn(Tower::new(team_point.truncate(), team));
            }
        }
    }
}

fn zig_zag(points: Vec<f32>, zig_first: bool, zig_spacing: f32) -> Vec<Vec2> {
    let mut zig_zag = Vec::new();
    for (i, point) in points.into_iter().enumerate() {
        zig_zag.push(Vec2::new(
            point,
            if (i % 2 == 0) == zig_first { 1. } else { -1. } * zig_spacing,
        ));
    }
    zig_zag
}

fn update() {}

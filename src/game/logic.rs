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

//multiply Vec3s in X_TRANSFORM space, by X_TRANSFORM, to convert them to GLOBAL space
pub static RED_TRANSFORM: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_translation(Vec2::splat(-1000.).extend(0.)));
pub static BLUE_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| {
    Transform::from_translation(Vec2::splat(1000.).extend(0.))
        .with_rotation(Quat::from_rotation_z(PI))
});
//multiply Vec3s in GLOBAL space, by X_TRANSFORM_INVERSE, to convert them to X_TRANSFORM space
pub static RED_TRANSFORM_INVERSE: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_matrix(RED_TRANSFORM.compute_matrix().inverse()));
pub static BLUE_TRANSFORM_INVERSE: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_matrix(BLUE_TRANSFORM.compute_matrix().inverse()));

fn team_transform(team: Team, inverse: bool) -> Transform {
    match team {
        Team::Red => {
            if inverse {
                *RED_TRANSFORM_INVERSE
            } else {
                *RED_TRANSFORM
            }
        }
        Team::Blue => {
            if inverse {
                *BLUE_TRANSFORM_INVERSE
            } else {
                *BLUE_TRANSFORM
            }
        }
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
        let height = 30.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(height / 2. + radius),
            )),
            radius: Radius(radius),
            health: Health(200.),
            team,
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Capsule(Capsule3d::new(radius, height)),
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
        let radius = 50.;
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

#[derive(Bundle)]
struct Monster {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    display: graphics::Display,
}
impl Monster {
    fn new(pos: Vec2) -> Self {
        let radius = 10.;
        let height = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(height / 2. + radius),
            )),
            radius: Radius(radius),
            health: Health(150.),
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Capsule(Capsule3d::new(radius, height)),
                color: graphics::NO_TEAM_COLOR,
                wireframe: false,
            },
        }
    }
}

#[derive(Bundle)]
struct Demon {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    display: graphics::Display,
}
impl Demon {
    fn new(pos: Vec2) -> Self {
        let radius = 25.;
        let height = 70.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(height / 2. + radius),
            )),
            radius: Radius(radius),
            health: Health(150.),
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Capsule(Capsule3d::new(radius, height)),
                color: graphics::NO_TEAM_COLOR,
                wireframe: false,
            },
        }
    }
}

fn init(mut commands: Commands) {
    spawn_cores(&mut commands);
    spawn_spawners(&mut commands);
    spawn_towers(&mut commands);
    spawn_advocates(&mut commands);
    spawn_monsters(&mut commands);
}

fn spawn_cores(commands: &mut Commands) {
    for team in [Team::Red, Team::Blue] {
        let point = Vec2::splat(300.).extend(0.);
        let team_point = team_transform(team, false) * point;
        commands.spawn(Core::new(team_point.truncate(), team));
    }
}

fn spawn_spawners(commands: &mut Commands) {
    let origin = Vec2::splat(OUTER_WALL_WIDTH + LANE_WIDTH / 2.).extend(0.);
    for team in [Team::Red, Team::Blue] {
        for lane in [Lane::Top, Lane::Mid, Lane::Bot] {
            let ang = PI / 4. * lane as i32 as f32;
            let point = Vec2::new(350., 0.).extend(0.);
            let ang_point = Transform::from_rotation(Quat::from_rotation_z(ang)) * point;
            let origin_point = ang_point + origin;
            let team_point = team_transform(team, false) * origin_point;
            commands.spawn(Spawner::new(team_point.truncate(), team));
        }
    }
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
                vec![500., 830., 1350.]
            };
            for point in zig_zag(points, zig, zig_spacing) {
                let point = point.extend(0.);
                let ang_point = Transform::from_rotation(Quat::from_rotation_z(ang)) * point;
                let origin_point = ang_point + origin;
                let team_point = team_transform(team, false) * origin_point;
                commands.spawn(Tower::new(team_point.truncate(), team));
            }
        }
    }
}

fn spawn_advocates(commands: &mut Commands) {
    let origin = Vec2::splat(200.).extend(0.);
    for team in [Team::Red, Team::Blue] {
        for advocate in 0..NUM_ADVOCATES {
            let ang = ((2. * PI) / NUM_ADVOCATES as f32) * advocate as f32;
            let point = Vec2::new(0., 50.).extend(0.);
            let ang_point = Transform::from_rotation(Quat::from_rotation_z(ang)) * point;
            let origin_point = ang_point + origin;
            let team_point = team_transform(team, false) * origin_point;
            commands.spawn(Advocate::new(team_point.truncate(), team));
        }
    }
}

fn spawn_monsters(commands: &mut Commands) {
    for team in [Team::Red, Team::Blue] {
        for point in vec![
            Vec2::new(1129., 351.),
            Vec2::new(1047., 527.),
            Vec2::new(949., 714.),
            Vec2::new(521., 878.),
            Vec2::new(512., 1066.),
            Vec2::new(293., 1129.),
        ] {
            let point = point.extend(0.);
            let team_point = team_transform(team, false) * point;
            commands.spawn(Monster::new(team_point.truncate()));
        }
        let point = Vec2::new(1341., 586.).extend(0.);
        let team_point = team_transform(team, false) * point;
        commands.spawn(Demon::new(team_point.truncate()));
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

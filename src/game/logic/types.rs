use crate::game::*;
use bevy::prelude::*;

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
pub struct Spawner {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Spawner {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(-radius),
            )),
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
pub struct Tower {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Tower {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let half_height = 30.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(logic::TOWER_RADIUS),
            health: Health(500.),
            team,
            display: graphics::Display {
                allowed_mesh: graphics::AllowedMesh::Cylinder(Cylinder::new(
                    logic::TOWER_RADIUS,
                    half_height * 2.,
                )),
                color: graphics::team_color(team),
                wireframe: false,
            },
        }
    }
}

#[derive(Bundle)]
pub struct Minion {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Minion {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 8.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
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
pub struct Advocate {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Advocate {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 12.;
        let height = 30.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
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
pub struct Core {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: graphics::Display,
}
impl Core {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 50.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                pos.extend(-radius),
            )),
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
pub struct Monster {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    display: graphics::Display,
}
impl Monster {
    pub fn new(pos: Vec2) -> Self {
        let radius = 10.;
        let height = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
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
pub struct Demon {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    display: graphics::Display,
}
impl Demon {
    pub fn new(pos: Vec2) -> Self {
        let radius = 25.;
        let height = 70.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
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

use super::*;
use bevy::prelude::*;
use graphics::{healthbar::DisplayHealthbar, model::*};

#[derive(Component, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}
impl Health {
    fn new(health: f32) -> Self {
        Self {
            current: health,
            maximum: health,
        }
    }
    fn with_current(&self, health: f32) -> Self {
        Self {
            current: health,
            maximum: self.maximum,
        }
    }
}

#[derive(Component)]
pub struct Radius(pub f32);

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
pub struct Core {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Core {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 50.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health::new(3400.).with_current(2100.),
            team,
            model: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                height: radius * 2.,
                raised: false,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(false),
        }
    }
}

#[derive(Bundle)]
pub struct Spawner {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Spawner {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health::new(901.).with_current(900.),
            team,
            model: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                height: radius * 2.,
                raised: false,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(false),
        }
    }
}

#[derive(Bundle)]
pub struct Tower {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Tower {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(logic::TOWER_RADIUS),
            health: Health::new(500.),
            team,
            model: DisplayModel {
                mesh_type: HashableMeshType::Cylinder,
                raised: true,
                height: 60.,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(false),
        }
    }
}

#[derive(Bundle)]
pub struct Advocate {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Advocate {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(12.),
            health: Health::new(200.),
            team,
            model: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                raised: true,
                height: 42.,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(false),
        }
    }
}

#[derive(Bundle)]
pub struct Minion {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Minion {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 8.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health::new(100.),
            team,
            model: DisplayModel {
                mesh_type: HashableMeshType::Cuboid,
                raised: true,
                height: radius * 2.,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(true),
        }
    }
}

#[derive(Bundle)]
pub struct Monster {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Monster {
    pub fn new(pos: Vec2) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(10.),
            health: Health::new(150.),
            model: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                raised: true,
                height: 35.,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(true),
        }
    }
}

#[derive(Bundle)]
pub struct Demon {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    model: DisplayModel,
    bar: DisplayHealthbar,
}
impl Demon {
    pub fn new(pos: Vec2) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(25.),
            health: Health::new(1500.),
            model: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                raised: true,
                height: 95.,
                wireframe: false,
            },
            bar: DisplayHealthbar::new(false),
        }
    }
}

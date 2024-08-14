use super::*;
use bevy::prelude::*;
use graphics::model::*;

#[derive(Component)]
pub struct Health {
    current: f32,
    maximum: f32,
}
impl Health {
    fn new(health: f32) -> Self {
        Self {
            current: health,
            maximum: health,
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
    display: DisplayModel,
}
impl Core {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 50.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health::new(5000.),
            team,
            display: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                height: radius * 2.,
                raised: false,
                wireframe: false,
            },
        }
    }
}

#[derive(Bundle)]
pub struct Spawner {
    spatial: SpatialBundle,
    radius: Radius,
    health: Health,
    team: Team,
    display: DisplayModel,
}
impl Spawner {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health::new(1000.),
            team,
            display: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                height: radius * 2.,
                raised: false,
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
    display: DisplayModel,
}
impl Tower {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(logic::TOWER_RADIUS),
            health: Health::new(500.),
            team,
            display: DisplayModel {
                mesh_type: HashableMeshType::Cylinder,
                raised: true,
                height: 60.,
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
    display: DisplayModel,
}
impl Advocate {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(12.),
            health: Health::new(200.),
            team,
            display: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                raised: true,
                height: 42.,
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
    display: DisplayModel,
}
impl Minion {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 8.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health::new(100.),
            team,
            display: DisplayModel {
                mesh_type: HashableMeshType::Cuboid,
                raised: true,
                height: radius * 2.,
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
    display: DisplayModel,
}
impl Monster {
    pub fn new(pos: Vec2) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(10.),
            health: Health::new(150.),
            display: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                raised: true,
                height: 35.,
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
    display: DisplayModel,
}
impl Demon {
    pub fn new(pos: Vec2) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(25.),
            health: Health::new(150.),
            display: DisplayModel {
                mesh_type: HashableMeshType::Capsule,
                raised: true,
                height: 95.,
                wireframe: false,
            },
        }
    }
}

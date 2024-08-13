use super::*;
use bevy::prelude::*;
use graphics::OrderedMeshType;

#[derive(Component)]
pub struct Health(f32);
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
    display: graphics::RenderComponent,
}
impl Core {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 50.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health(5000.),
            team,
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Capsule,
                height: radius * 2.,
                color: graphics::team_color(Some(team)),
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
    display: graphics::RenderComponent,
}
impl Spawner {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 25.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health(1000.),
            team,
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Capsule,
                height: radius * 2.,
                color: graphics::team_color(Some(team)).with_alpha(0.7),
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
    display: graphics::RenderComponent,
}
impl Tower {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(logic::TOWER_RADIUS),
            health: Health(500.),
            team,
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Cylinder,
                color: graphics::team_color(Some(team)),
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
    display: graphics::RenderComponent,
}
impl Advocate {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(12.),
            health: Health(200.),
            team,
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Capsule,
                color: graphics::team_color(Some(team)),
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
    display: graphics::RenderComponent,
}
impl Minion {
    pub fn new(pos: Vec2, team: Team) -> Self {
        let radius = 8.;
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(radius),
            health: Health(100.),
            team,
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Cuboid,
                color: graphics::team_color(Some(team)),
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
    display: graphics::RenderComponent,
}
impl Monster {
    pub fn new(pos: Vec2) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(10.),
            health: Health(150.),
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Capsule,
                color: graphics::team_color(None),
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
    display: graphics::RenderComponent,
}
impl Demon {
    pub fn new(pos: Vec2) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            radius: Radius(25.),
            health: Health(150.),
            display: graphics::RenderComponent {
                mesh_type: OrderedMeshType::Capsule,
                color: graphics::team_color(None),
                raised: true,
                height: 95.,
                wireframe: false,
            },
        }
    }
}

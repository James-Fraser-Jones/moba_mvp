use crate::game::*;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Health(pub f32);

#[derive(Component, Clone, Copy)]
pub struct MaxHealth(pub f32);

#[derive(Component, Clone, Copy)]
pub struct MovePosition(pub Option<Vec2>);

#[derive(Component, Clone, Copy)]
pub struct MoveSpeed(pub f32);

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

// pub enum PlayerAction {
//     Move,
//     Attack,
//     Cast,
//     Level,
//     Interact,
// }

#[derive(Bundle)]
pub struct MoveBundle {
    move_position: MovePosition,
    move_speed: MoveSpeed,
}
impl MoveBundle {
    pub fn new(move_speed: f32) -> Self {
        Self {
            move_position: MovePosition(None),
            move_speed: MoveSpeed(move_speed),
        }
    }
}

#[derive(Bundle)]
pub struct ModelBundle {
    radius: Radius,
    model: DisplayModel,
}
impl ModelBundle {
    pub fn new(radius: f32, model: DisplayModel) -> Self {
        Self {
            radius: Radius(radius),
            model,
        }
    }
}

#[derive(Bundle)]
pub struct HealthBundle {
    health: Health,
    max_health: MaxHealth,
    display: DisplayHealthbar,
}
impl HealthBundle {
    pub fn new(health: f32, display: DisplayHealthbar) -> Self {
        Self {
            health: Health(health),
            max_health: MaxHealth(health),
            display,
        }
    }
}

#[derive(Bundle)]
pub struct Unit {
    spatial: SpatialBundle,
    model: ModelBundle,
    health: HealthBundle,
}
impl Unit {
    pub fn new(
        pos: Vec2,
        radius: f32,
        model: DisplayModel,
        health: f32,
        healthbar: DisplayHealthbar,
    ) -> Self {
        Self {
            spatial: new_spatial(pos),
            model: ModelBundle::new(radius, model),
            health: HealthBundle::new(health, healthbar),
        }
    }
}
fn new_spatial(pos: Vec2) -> SpatialBundle {
    SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.)))
}

#[derive(Bundle)]
pub struct Core {
    unit: Unit,
    team: Team,
}
impl Core {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: Unit::new(
                pos,
                50.,
                DisplayModel::hemisphere(),
                3400.,
                DisplayHealthbar::Advanced,
            ),
            team,
        }
    }
}

#[derive(Bundle)]
pub struct Spawner {
    unit: Unit,
    team: Team,
}
impl Spawner {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: Unit::new(
                pos,
                25.,
                DisplayModel::hemisphere(),
                900.,
                DisplayHealthbar::Advanced,
            ),
            team,
        }
    }
}

pub const TOWER_RADIUS: f32 = 20.;
#[derive(Bundle)]
pub struct Tower {
    unit: Unit,
    team: Team,
}
impl Tower {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: Unit::new(
                pos,
                TOWER_RADIUS,
                DisplayModel::cylinder().with_height_ratio(1.5),
                500.,
                DisplayHealthbar::Advanced,
            ),
            team,
        }
    }
}

//handle for player plugin to access
#[derive(Component)]
pub struct PlayerID(pub i32);

#[derive(Bundle)]
pub struct Advocate {
    unit: Unit,
    team: Team,
    move_: MoveBundle,
    player_id: PlayerID,
}
impl Advocate {
    pub fn new(pos: Vec2, team: Team, player_id: PlayerID) -> Self {
        Self {
            unit: Unit::new(
                pos,
                12.,
                DisplayModel::capsule().with_height_ratio(1.75),
                200.,
                DisplayHealthbar::Advanced,
            ),
            team,
            move_: MoveBundle::new(100.),
            player_id,
        }
    }
}

#[derive(Bundle)]
pub struct Minion {
    unit: Unit,
    team: Team,
    move_: MoveBundle,
}
impl Minion {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: Unit::new(pos, 8., DisplayModel::cube(), 100., DisplayHealthbar::Basic),
            team,
            move_: MoveBundle::new(50.),
        }
    }
}

#[derive(Bundle)]
pub struct Monster {
    unit: Unit,
    move_: MoveBundle,
}
impl Monster {
    pub fn new(pos: Vec2) -> Self {
        Self {
            unit: Unit::new(
                pos,
                10.,
                DisplayModel::capsule().with_height_ratio(1.75),
                150.,
                DisplayHealthbar::Basic,
            ),
            move_: MoveBundle::new(200.),
        }
    }
}

#[derive(Bundle)]
pub struct Demon {
    unit: Unit,
    move_: MoveBundle,
}
impl Demon {
    pub fn new(pos: Vec2) -> Self {
        Self {
            unit: Unit::new(
                pos,
                25.,
                DisplayModel::capsule().with_height_ratio(1.9),
                1500.,
                DisplayHealthbar::Advanced,
            ),
            move_: MoveBundle::new(300.),
        }
    }
}

//use crate::*;
use bevy::prelude::*;

//==========================================================
//================ IO Handle Components ====================

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum InputHandle {
    Input0,
    Input1,
    Input2,
    Input3,
    Input4,
    Input5,
    Input6,
    Input7,
    Input8,
    Input9,
}
impl From<u8> for InputHandle {
    fn from(value: u8) -> Self {
        match value {
            0 => InputHandle::Input0,
            1 => InputHandle::Input1,
            2 => InputHandle::Input2,
            3 => InputHandle::Input3,
            4 => InputHandle::Input4,
            5 => InputHandle::Input5,
            6 => InputHandle::Input6,
            7 => InputHandle::Input7,
            8 => InputHandle::Input8,
            9 => InputHandle::Input9,
            _ => panic!(),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum OutputHandle {
    Core,
    Spawner,
    Tower,
    Advocate,
    Minion,
    Monster,
    Demon,
}

//==========================================================
//====================== Components ========================

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Health(pub f32);

#[derive(Component, Clone, Copy, PartialEq)]
pub struct MaxHealth(pub f32);

#[derive(Component, Clone, Copy, PartialEq)]
pub struct MovePosition(pub Option<Vec2>);

#[derive(Component, Clone, Copy, PartialEq)]
pub struct MoveSpeed(pub f32);

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Radius(pub f32);

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Red,
    Blue,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    Bot,
    Mid,
    Top,
}

//==========================================================
//====================== Bundles ===========================

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
pub struct HealthBundle {
    health: Health,
    max_health: MaxHealth,
}
impl HealthBundle {
    pub fn new(health: Health) -> Self {
        Self {
            health,
            max_health: MaxHealth(health.0),
        }
    }
}

#[derive(Bundle)]
pub struct UnitBundle {
    spatial: SpatialBundle,
    radius: Radius,
    health: HealthBundle,
}
impl UnitBundle {
    pub fn new(pos: Vec2, radius: Radius, health: Health) -> Self {
        Self {
            spatial: spatial_from_pos(pos),
            radius,
            health: HealthBundle::new(health),
        }
    }
}
fn spatial_from_pos(pos: Vec2) -> SpatialBundle {
    SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.)))
}

//==========================================================
//================= Top Level Bundles ======================

#[derive(Bundle)]
pub struct CoreBundle {
    unit: UnitBundle,
    team: Team,
    output: OutputHandle,
}
impl CoreBundle {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: UnitBundle::new(pos, Radius(50.), Health(3400.)),
            team,
            output: OutputHandle::Core,
        }
    }
}

#[derive(Bundle)]
pub struct SpawnerBundle {
    unit: UnitBundle,
    team: Team,
    output: OutputHandle,
}
impl SpawnerBundle {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: UnitBundle::new(pos, Radius(25.), Health(900.)),
            team,
            output: OutputHandle::Spawner,
        }
    }
}

pub const TOWER_RADIUS: Radius = Radius(20.);
#[derive(Bundle)]
pub struct TowerBundle {
    unit: UnitBundle,
    team: Team,
    output: OutputHandle,
}
impl TowerBundle {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: UnitBundle::new(pos, TOWER_RADIUS, Health(500.)),
            team,
            output: OutputHandle::Tower,
        }
    }
}

#[derive(Bundle)]
pub struct AdvocateBundle {
    unit: UnitBundle,
    team: Team,
    move_: MoveBundle,
    output: OutputHandle,
    input: InputHandle,
}
impl AdvocateBundle {
    pub fn new(pos: Vec2, team: Team, input: InputHandle) -> Self {
        Self {
            unit: UnitBundle::new(pos, Radius(12.), Health(200.)),
            team,
            move_: MoveBundle::new(100.),
            output: OutputHandle::Advocate,
            input,
        }
    }
}

#[derive(Bundle)]
pub struct MinionBundle {
    unit: UnitBundle,
    team: Team,
    move_: MoveBundle,
    output: OutputHandle,
}
impl MinionBundle {
    pub fn new(pos: Vec2, team: Team) -> Self {
        Self {
            unit: UnitBundle::new(pos, Radius(8.), Health(100.)),
            team,
            move_: MoveBundle::new(50.),
            output: OutputHandle::Minion,
        }
    }
}

#[derive(Bundle)]
pub struct MonsterBundle {
    unit: UnitBundle,
    move_: MoveBundle,
    output: OutputHandle,
}
impl MonsterBundle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            unit: UnitBundle::new(pos, Radius(10.), Health(150.)),
            move_: MoveBundle::new(200.),
            output: OutputHandle::Monster,
        }
    }
}

#[derive(Bundle)]
pub struct DemonBundle {
    unit: UnitBundle,
    move_: MoveBundle,
    output: OutputHandle,
}
impl DemonBundle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            unit: UnitBundle::new(pos, Radius(25.), Health(1500.)),
            move_: MoveBundle::new(300.),
            output: OutputHandle::Demon,
        }
    }
}

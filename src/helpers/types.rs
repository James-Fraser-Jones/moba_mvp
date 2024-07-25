use crate::helpers::{consts::*, utils::*};
use bevy::prelude::*;

//================================================================================
// Generic Components ============================================================
//================================================================================

//movetype
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum MoveType {
    #[default]
    Stationary,
    Move(Vec2),
    AttackMove(Vec2),
    Attack(Entity),
}

//discipline
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum Discipline {
    #[default]
    Melee,
    Ranged,
}

//lane
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum Lane {
    #[default]
    Mid,
    Top,
    Bot,
}

//team
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum Team {
    #[default]
    Red,
    Blue,
}

#[derive(Component, Default)] //Timer that is advanced predictably in the FixedUpdate logic cycle
pub struct FixedTimer(pub Timer);

//================================================================================
// Specific Components ===========================================================
//================================================================================

#[derive(Component, Default)] //tracks whether minion has crossed mid-point of its assigned lane
pub struct MidCrossed(pub bool);

#[derive(Component, Default)] //tracks the position of a unit at the end of the previous frame
pub struct OldPos(pub Vec2);

//================================================================================
// Resources =====================================================================
//================================================================================

//wave manager
#[derive(Resource, Default)]
pub struct WaveManager {
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub spawn_index: i32,
}
impl WaveManager {
    pub fn new() -> Self {
        Self {
            wave_timer: Timer::from_seconds(WAVE_DELAY, TimerMode::Repeating),
            spawn_timer: Timer::from_seconds(SPAWNER_DELAY, TimerMode::Repeating),
            ..default()
        }
    }
}

//================================================================================
// Bundles =======================================================================
//================================================================================

//map
#[derive(Component, Default)]
pub struct Map;
#[derive(Bundle, Default)]
pub struct MapBundle {
    pub spatial: SpatialBundle,
    pub map: Map,
}
impl MapBundle {
    pub fn new() -> Self {
        Self::default()
    }
}

//spawner
#[derive(Component, Default)]
pub struct Spawner;
#[derive(Bundle, Default)]
pub struct SpawnerBundle {
    pub spatial: SpatialBundle,
    pub team: Team,
    pub lane: Lane,
    pub spawner: Spawner,
}
impl SpawnerBundle {
    pub fn new(vec4: Vec4, team: Team, lane: Lane) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(vec4)),
            team,
            lane,
            ..default()
        }
    }
}

//unit
#[derive(Component, Default)]
pub struct Unit;
#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub spatial: SpatialBundle,
    pub team: Team,
    pub discipline: Discipline,
    pub lane: Lane,
    pub move_type: MoveType,
    pub mid_crossed: MidCrossed,
    pub old_pos: OldPos,
    pub unit: Unit, //tag for query filtering
}
impl UnitBundle {
    pub fn new(vec4: Vec4, team: Team, discipline: Discipline, lane: Lane) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(vec4)),
            team,
            discipline,
            lane,
            ..default()
        }
    }
}

//================================================================================
// Events ========================================================================
//================================================================================

#[derive(Event)]
pub struct GraphicsEvent {
    pub entity: Entity,
}
impl GraphicsEvent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

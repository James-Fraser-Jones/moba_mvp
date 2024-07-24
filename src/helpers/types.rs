use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::helpers::consts::*;

use super::utils::vec4_to_trans;
use std::f32::consts::PI;

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

//unit
#[derive(Component, Default)]
pub struct Unit;

#[derive(Bundle, Default)]
pub struct UnitBundle {
    pub spatial: SpatialBundle,
    pub team: Team,
    pub discipline: Discipline,
    pub lane: Lane,
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

//spawner
#[derive(Component, Default)]
pub struct FixedTimer(pub Timer);

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

//asset handles
#[derive(Resource)]
pub struct Handles {
    pub unit: Mesh2dHandle,
    pub direction: Mesh2dHandle,
    pub spawner: Mesh2dHandle,
    pub plain: Mesh2dHandle,
    pub river: Mesh2dHandle,
    pub mid: Mesh2dHandle,
    pub lane: Mesh2dHandle,
    pub base: Mesh2dHandle,
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub teal: Handle<ColorMaterial>,
    pub purple: Handle<ColorMaterial>,
}

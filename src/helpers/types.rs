use bevy::{prelude::*, sprite::Mesh2dHandle};

//team
#[derive(Component, PartialEq, Default, Copy, Clone)]
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
    pub unit: Unit, //tag for query filtering
}
impl UnitBundle {
    pub fn from_xyrt(x: f32, y: f32, r: f32, t: Team) -> Self {
        Self {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.).with_rotation(Quat::from_rotation_z(r)),
                ..default()
            },
            team: t,
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
    pub timer: FixedTimer,
    pub spawner: Spawner,
}
impl SpawnerBundle {
    pub fn from_xyrt(x: f32, y: f32, r: f32, t: Team, s: f32) -> Self {
        Self {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.).with_rotation(Quat::from_rotation_z(r)),
                ..default()
            },
            team: t,
            timer: FixedTimer(Timer::from_seconds(s, TimerMode::Repeating)),
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

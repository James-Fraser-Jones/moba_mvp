use crate::game::graphics::MeshBundle;
use crate::game::{consts::*, utils::*};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::f32::consts::PI;

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

//root
#[derive(Component, Default)]
pub struct Root;
#[derive(Bundle, Default)]
pub struct RootBundle {
    pub spatial: SpatialBundle,
    pub root: Root,
}
impl RootBundle {
    pub fn new() -> Self {
        Self { ..default() }
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands.spawn(self).id()
    }
}

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
    pub fn spawn(self, root: &mut EntityCommands) -> Entity {
        let map = root
            .commands()
            .spawn(self)
            .with_children(|builder| {
                builder.spawn(MeshBundle::new(
                    "plain",
                    "green",
                    vec4_to_trans(Vec4::new(0., 0., -5., 0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "river",
                    "teal",
                    vec4_to_trans(Vec4::new(0., 0., -4., PI / 4.)),
                ));
                builder.spawn(MeshBundle::new(
                    "mid",
                    "yellow",
                    vec4_to_trans(Vec4::new(0., 0., -3., -PI / 4.)),
                ));
                //red top
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(Vec4::new(-MID_LANE * MAP_SIZE, 0., -3., 0.)),
                ));
                //blue top
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(Vec4::new(0., MID_LANE * MAP_SIZE, -3., 2. * PI / 4.)),
                ));
                //red bot
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(Vec4::new(0., -MID_LANE * MAP_SIZE, -3., 2. * PI / 4.)),
                ));
                //blue bot
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(Vec4::new(MID_LANE * MAP_SIZE, 0., -3., 0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "base",
                    "red",
                    vec4_to_trans(Vec4::new(-MAP_SIZE / 2., -MAP_SIZE / 2., -2., -PI / 4.)),
                ));
                builder.spawn(MeshBundle::new(
                    "base",
                    "blue",
                    vec4_to_trans(Vec4::new(MAP_SIZE / 2., MAP_SIZE / 2., -2., 3. * PI / 4.)),
                ));
            })
            .id();
        root.add_child(map);
        map
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
    pub fn spawn(self, root: &mut EntityCommands) -> Entity {
        let spawner = root
            .commands()
            .spawn(self)
            .with_children(|builder| {
                builder.spawn(MeshBundle::new(
                    "spawner",
                    "purple",
                    vec4_to_trans(Vec4::new(0., 0., 0., 0.)),
                ));
            })
            .id();
        root.add_child(spawner);
        spawner
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
    pub fn spawn(self, root: &mut EntityCommands) -> Entity {
        let team = match self.team {
            Team::Red => "red",
            Team::Blue => "blue",
        };
        let unit = root
            .commands()
            .spawn(self)
            .with_children(|builder| {
                builder.spawn(MeshBundle::new(
                    "unit",
                    "green",
                    vec4_to_trans(Vec4::new(0., 0., 0., 0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "direction",
                    team,
                    vec4_to_trans(Vec4::new(0., 0., 1., 0.)),
                ));
            })
            .id();
        root.add_child(unit);
        unit
    }
}

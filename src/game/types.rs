use crate::game::{consts::*, graphics::MeshBundle, utils::*};
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use std::f32::consts::PI;

//================================================================================
// Generic Components ============================================================
//================================================================================

//=======================================
// Behaviour enums

//attack override
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum AttackOverride {
    Ignore, //Unit will not be distracted from current action
    #[default]
    Attack, //Unit will be distracted from current action to attack enemies within attack range
}

//attack behaviour
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum AttackBehaviour {
    #[default]
    Pursue, //Unit pursues target to within attack range
    Attack, //Unit attacks on cooldown
}

//movetype
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Action {
    Stop(AttackOverride),            //Unit remains stationary
    Move(Pos, AttackOverride),       //Unit moves to the location
    Attack(Entity, AttackBehaviour), //Unit moves within attack range of enemy
}
impl Default for Action {
    fn default() -> Self {
        Self::Stop(AttackOverride::Attack)
    }
}

//lane
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum Lane {
    #[default]
    Mid,
    Top,
    Bot,
}

//=======================================
// Type enums

//discipline
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum Discipline {
    #[default]
    Melee,
    Ranged,
}

//team
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum Team {
    #[default]
    Red,
    Blue,
}

//=======================================
// Misc

//Timer that is advanced predictably in the FixedUpdate logic cycle
#[derive(Component, Default)]
pub struct FixedTimer(pub Timer);

//=======================================
// Space-logical newtypes

#[derive(Component, Default, Copy, Clone, PartialEq, Debug)]
pub struct Pos(pub Vec2);
impl Pos {
    pub fn from_transform(trans: &Transform) -> Self {
        Pos(trans.translation.truncate())
    }
    pub fn set_transform(&self, trans: &mut Transform) {
        trans.translation.x = self.0.x;
        trans.translation.y = self.0.y;
    }
}

//================================================================================
// Specific Components ===========================================================
//================================================================================

//tracks whether minion has crossed mid-point of its assigned lane
//eventually will be replaced with some kind of Path2D equivalent
#[derive(Component, Default)]
pub struct MidCrossed(pub bool);

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
    //rendering
    pub spatial: SpatialBundle,
    //tag
    pub map: Map,
}
impl MapBundle {
    pub fn new() -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(
                Vec2::ZERO.extend(-6.).extend(0.),
            )),
            ..default()
        }
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(self)
            .with_children(|builder| {
                builder.spawn(MeshBundle::new(
                    "plain",
                    "dark_green",
                    vec4_to_trans(MID.extend(0.).extend(0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "river",
                    "teal",
                    vec4_to_trans(MID.extend(2.5).extend(PI / 4.)),
                ));
                builder.spawn(MeshBundle::new(
                    "mid",
                    "yellow",
                    vec4_to_trans(MID.extend(5.).extend(-PI / 4.)),
                ));
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(RED_TOP.extend(5.).extend(0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(BLUE_TOP.extend(5.).extend(PI / 2.)),
                ));
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(RED_BOT.extend(5.).extend(PI / 2.)),
                ));
                builder.spawn(MeshBundle::new(
                    "lane",
                    "yellow",
                    vec4_to_trans(BLUE_BOT.extend(5.).extend(0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "base",
                    "dark_red",
                    vec4_to_trans(Vec4::new(-1000., -1000., 6., -PI / 4.)),
                ));
                builder.spawn(MeshBundle::new(
                    "base",
                    "dark_blue",
                    vec4_to_trans(Vec4::new(1000., 1000., 6., 3. * PI / 4.)),
                ));
            })
            .id()
    }
}

//spawner
#[derive(Component, Default)]
pub struct Spawner;
#[derive(Bundle, Default)]
pub struct SpawnerBundle {
    //rendering
    pub spatial: SpatialBundle,
    //type
    pub team: Team,
    pub lane: Lane,
    //tag
    pub spawner: Spawner,
}
impl SpawnerBundle {
    pub fn new(pos: Pos, team: Team, lane: Lane) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(pos.0.extend(0.).extend(0.))),
            team,
            lane,
            ..default()
        }
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(self)
            .with_children(|builder| {
                builder.spawn(MeshBundle::new(
                    "spawner",
                    "purple",
                    vec4_to_trans(Vec4::new(0., 0., SPAWNER_RADIUS, 0.)),
                ));
            })
            .id()
    }
}

//unit
#[derive(Component, Default)]
pub struct Unit;
#[derive(Bundle, Default)]
pub struct UnitBundle {
    //rendering
    pub spatial: SpatialBundle,
    //type
    pub team: Team,
    pub discipline: Discipline,
    //behaviour
    pub action: Action,
    pub mid_crossed: MidCrossed,
    pub attack_timer: FixedTimer,
    //physics
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub locked_axes: LockedAxes,
    pub friction: Friction,
    //tag
    pub unit: Unit,
}
impl UnitBundle {
    pub fn new(pos: Pos, team: Team, discipline: Discipline, action: Action) -> Self {
        let mut trans = Transform::IDENTITY;
        pos.set_transform(&mut trans);
        Self {
            spatial: SpatialBundle::from_transform(trans),
            team,
            discipline,
            action,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::circle(UNIT_RADIUS as Scalar),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction::ZERO,
            ..default()
        }
    }
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let team = match self.team {
            Team::Red => "red",
            Team::Blue => "blue",
        };
        commands
            .spawn(self)
            .with_children(|builder| {
                builder.spawn(MeshBundle::new(
                    "unit",
                    "green_trans",
                    vec4_to_trans(Vec4::new(0., 0., UNIT_RADIUS, 0.)),
                ));
                builder.spawn(MeshBundle::new(
                    "direction",
                    team,
                    vec4_to_trans(Vec4::new(
                        UNIT_RADIUS * (1. - UNIT_TRIANGLE_ANGLE.cos().powf(2.)),
                        0.,
                        UNIT_RADIUS,
                        -PI / 2.,
                    )),
                ));
            })
            .id()
    }
}

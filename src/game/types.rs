use crate::game::consts::*;
use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

//================================================================================
// Generic Components ============================================================
//================================================================================

//=======================================
// Behaviour enums

//attack override
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum AttackOverride {
    Ignore, //Unit will not be distracted from current action
    Attack, //Unit will be distracted from current action to attack enemies within attack range
}

//attack behaviour
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum AttackBehaviour {
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
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Lane {
    Top,
    Mid,
    Bot,
}

//range
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Range {
    Collide,
    Attack,
    Sight,
}

//=======================================
// Type enums

//discipline
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Discipline {
    Melee,
    Ranged,
}

//team
#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Team {
    Red,
    Blue,
}

//=======================================
// Physics enums

//collision layer
#[derive(PartialEq, Copy, Clone, Debug, PhysicsLayer)]
pub enum CollisionLayer {
    RedUnit,
    BlueUnit,
    RedAttack,
    BlueAttack,
    RedSight,
    BlueSight,
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
        Self {
            spatial: SpatialBundle::from_transform(Transform::from_translation(
                Vec3::ZERO.with_z(-6.),
            )),
            ..default()
        }
    }
}

//spawner
#[derive(Component)]
pub struct Spawner {
    pub lane: Lane,
}
#[derive(Bundle)]
pub struct SpawnerBundle {
    pub spawner: Spawner,
    pub spatial: SpatialBundle,
    pub team: Team,
}
impl SpawnerBundle {
    pub fn new(pos: Pos, team: Team, lane: Lane) -> Self {
        Self {
            spawner: Spawner { lane },
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.0.extend(0.))),
            team,
        }
    }
}

//unit
#[derive(Component)]
pub struct Unit {
    pub discipline: Discipline,
    pub action: Action,
    pub mid_crossed: bool,
}
#[derive(Bundle)]
pub struct UnitBundle {
    pub unit: Unit, //unit-specific component
    pub spatial: SpatialBundle,
    pub team: Team,
    //physics
    pub attack_timer: FixedTimer,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub locked_axes: LockedAxes,
    pub friction: Friction,
}
impl UnitBundle {
    pub fn new(pos: Pos, team: Team, discipline: Discipline, action: Action) -> Self {
        let unit_layer = match team {
            Team::Red => CollisionLayer::RedUnit,
            Team::Blue => CollisionLayer::BlueUnit,
        };
        let opposite_sight = match team {
            Team::Red => CollisionLayer::BlueSight,
            Team::Blue => CollisionLayer::RedSight,
        };
        let opposite_attack = match team {
            Team::Red => CollisionLayer::BlueAttack,
            Team::Blue => CollisionLayer::RedAttack,
        };
        let mut trans = Transform::IDENTITY;
        pos.set_transform(&mut trans);
        Self {
            unit: Unit {
                discipline,
                action,
                mid_crossed: false,
            },
            spatial: SpatialBundle::from_transform(trans),
            team,
            rigidbody: RigidBody::Dynamic,
            collider: Collider::circle(UNIT_RADIUS as Scalar),
            collision_layers: CollisionLayers::new(
                unit_layer,
                [
                    CollisionLayer::RedUnit,
                    CollisionLayer::BlueUnit,
                    opposite_sight,
                    opposite_attack,
                ],
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction::ZERO,
            attack_timer: FixedTimer::default(),
        }
    }
}

//collider tags so they can be discerned from each other in queries
#[derive(Component, Default)]
pub struct SightCollider;
#[derive(Component, Default)]
pub struct AttackCollider;

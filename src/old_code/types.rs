use avian2d::prelude::*;
use bevy::prelude::*;

//===============SETTINGS==================

#[derive(Resource)]
pub struct MapSettings {
    lane_width: f32,
    base_radius: f32,
}
impl Default for MapSettings {
    fn default() -> Self {
        Self {
            lane_width: 240.,
            base_radius: 360.,
        }
    }
}
impl MapSettings {
    fn non_lane_radius(&self) -> f32 {
        1000. - self.lane_width
    }
    fn mid_lane_radius(&self) -> f32 {
        1000. - self.lane_width / 2.
    }
    fn location(&self, lane: Lane, team: Team) -> Vec2 {
        Vec2::ZERO
    }
}

#[derive(Resource)]
pub struct UnitSettings {
    radius: f32,
    speed: f32,
    sight_radius: f32,
    attack_radius: f32,
    attack_speed: f32,
    health: f32,
    attack_damage: f32,
}
impl Default for UnitSettings {
    fn default() -> Self {
        Self {
            radius: 18.5,
            speed: 185.,
            sight_radius: 55.6,
            attack_radius: 37.,
            attack_speed: 1.,
            health: 100.,
            attack_damage: 10.,
        }
    }
}

#[derive(Resource)]
pub struct WaveSettings {
    wave_delay: f32,
    spawn_delay: f32,
    spawn_count: i32,
}
impl Default for WaveSettings {
    fn default() -> Self {
        Self {
            wave_delay: 20.,
            spawn_delay: 1.,
            spawn_count: 6,
        }
    }
}

//===============RESOURCES==================

#[derive(Resource)]
pub struct WaveManager {
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub spawn_count: i32,
    pub spawn_index: i32,
}
impl WaveManager {
    pub fn new(wave_delay: f32, spawn_delay: f32, spawn_count: i32) -> Self {
        Self {
            wave_timer: Timer::from_seconds(wave_delay, TimerMode::Repeating),
            spawn_timer: Timer::from_seconds(spawn_delay, TimerMode::Repeating),
            spawn_count,
            spawn_index: 0,
        }
    }
}

//==============COMPONENTS===================

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Action {
    Stop(bool),           //Unit remains stationary, unless bool and target in vision
    Move(Vec2, bool),     //Unit moves to the location, unless bool and target in vision
    Attack(Entity, bool), //If bool, unit attacks, else, unit moves within attack range of target
}
impl Default for Action {
    fn default() -> Self {
        Self::Stop(true)
    }
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Lane {
    Top,
    Mid,
    Bot,
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Discipline {
    Melee,
    Ranged,
}

#[derive(Component, PartialEq, Copy, Clone, Debug)]
pub enum Team {
    Red,
    Blue,
}

//collision layer
#[derive(PartialEq, Copy, Clone, Debug, PhysicsLayer)]
pub enum CollisionLayer {
    RedUnit,
    BlueUnit,
    RedSight,
    BlueSight,
}

//Timer that is advanced predictably in the FixedUpdate logic cycle
#[derive(Component, Default)]
pub struct FixedTimer(pub Timer);

//spawner
#[derive(Component)]
pub struct Spawner {
    pub lane: Lane,
}

//unit
#[derive(Component)]
pub struct Unit {
    pub discipline: Discipline,
    pub action: Action,
    pub mid_crossed: bool,
    pub attack_timer: Timer,
}

//==============BUNDLES===================

#[derive(Bundle)]
pub struct Spawner {
    pub spatial: SpatialBundle,
    pub team: Team,
}
impl Spawner {
    pub fn new(pos: Vec2, team: Team, lane: Lane) -> Self {
        Self {
            spawner: Spawner { lane },
            spatial: SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.))),
            team,
        }
    }
}

#[derive(Bundle)]
pub struct Minion {
    pub spatial: SpatialBundle,
    pub team: Team,
    //physics
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub collision_layers: CollisionLayers,
    pub locked_axes: LockedAxes,
    pub friction: Friction,
}
impl Minion {
    pub fn new(pos: Vec2, team: Team, discipline: Discipline, action: Action) -> Self {
        let unit_layer = match team {
            Team::Red => CollisionLayer::RedUnit,
            Team::Blue => CollisionLayer::BlueUnit,
        };
        let opposite_sight = match team {
            Team::Red => CollisionLayer::BlueSight,
            Team::Blue => CollisionLayer::RedSight,
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
                ],
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction::ZERO,
            attack_timer: FixedTimer::default(),
        }
    }
}

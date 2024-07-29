use crate::game::{consts::*, graphics::MeshBundle, utils::*};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::collections::{HashMap, HashSet};
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
    Move(Position, AttackOverride),  //Unit moves to the location
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

//TODO: remove this and simply merge it directly into spatial index
#[derive(Component, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PositionIndex(pub IVec2);
impl PositionIndex {
    pub fn from_position(Position(position): Position) -> Self {
        PositionIndex((position / CELL_HALF_SIZE).as_ivec2())
    }
}

//=======================================
// Space-logical newtypes

//TODO: remove these and replace with some helper methods for accessing/modifying/updating the associated parts of transforms directly
#[derive(Component, Default, Copy, Clone, PartialEq, Debug)]
pub struct Position(pub Vec2);
impl Position {
    pub fn from_transform(trans: &Transform) -> Self {
        Position(trans.translation.truncate())
    }
    pub fn set_transform(&self, trans: &mut Transform) {
        trans.translation.x = self.0.x;
        trans.translation.y = self.0.y;
    }
}

#[derive(Component, Default, Copy, Clone, PartialEq)]
pub struct Orientation(pub f32);
impl Orientation {
    pub fn from_transform(trans: &Transform) -> Self {
        Orientation(trans.rotation.to_euler(EulerRot::XYZ).2)
    }
    pub fn set_transform(&self, trans: &mut Transform) {
        trans.rotation = Quat::from_rotation_z(self.0)
    }
}

#[derive(Component, Default, Copy, Clone, PartialEq)]
pub struct Radius(pub f32);
impl Radius {
    pub fn from_transform(trans: &Transform) -> Self {
        Radius((trans.scale.x + trans.scale.y) / 2.)
    }
    pub fn set_transform(&self, trans: &mut Transform) {
        trans.scale.x = self.0;
        trans.scale.y = self.0;
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

#[derive(Resource, Default)]
pub struct SpatialIndex(HashMap<PositionIndex, HashSet<Entity>>);
impl SpatialIndex {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_nearby_units(&self, position: PositionIndex, radius: Radius) -> Vec<Entity> {
        let radius_index = (radius.0 / (2. * CELL_HALF_SIZE)).ceil() as i32;
        let mut nearby = Vec::new();
        for x in -radius_index..=radius_index {
            for y in -radius_index..=radius_index {
                if let Some(units) = self.0.get(&(PositionIndex(position.0 + IVec2::new(x, y)))) {
                    nearby.extend(units.iter());
                }
            }
        }
        nearby
    }
    pub fn add_unit(&mut self, entity: Entity, pos: PositionIndex) {
        if let Some(set) = self.0.get_mut(&pos) {
            set.insert(entity);
        } else {
            self.0.insert(pos, HashSet::from([entity]));
        }
    }
    pub fn remove_unit(&mut self, entity: Entity, pos: PositionIndex) {
        self.0.get_mut(&pos).unwrap().remove(&entity);
    }
    pub fn move_unit(&mut self, entity: Entity, old_pos: PositionIndex, new_pos: PositionIndex) {
        if old_pos.0 != new_pos.0 {
            self.0.get_mut(&old_pos).unwrap().remove(&entity);
            if let Some(set) = self.0.get_mut(&new_pos) {
                set.insert(entity);
            } else {
                self.0.insert(new_pos, HashSet::from([entity]));
            }
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
    //rendering
    pub spatial: SpatialBundle,
    //tag
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
    pub fn spawn(self, root: &mut EntityCommands) -> Entity {
        let map = root
            .commands()
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
    //rendering
    pub spatial: SpatialBundle,
    //type
    pub team: Team,
    pub lane: Lane,
    //tag
    pub spawner: Spawner,
}
impl SpawnerBundle {
    pub fn new(position: Position, team: Team, lane: Lane) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(position.0.extend(0.).extend(0.))),
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
                    vec4_to_trans(Vec4::new(0., 0., SPAWNER_RADIUS, 0.)),
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
    //rendering
    pub spatial: SpatialBundle,
    //type
    pub team: Team,
    pub discipline: Discipline,
    //behaviour
    pub action: Action,
    pub mid_crossed: MidCrossed,
    pub attack_timer: FixedTimer,
    //space
    pub old_position: Position,
    //tag
    pub unit: Unit,
}
impl UnitBundle {
    pub fn new(position: Position, team: Team, discipline: Discipline, action: Action) -> Self {
        let mut trans = Transform::IDENTITY;
        position.set_transform(&mut trans);
        Self {
            spatial: SpatialBundle::from_transform(trans),
            team,
            discipline,
            action,
            old_position: position,
            ..default()
        }
    }
    pub fn spawn(
        self,
        root: &mut EntityCommands,
        spatial_index: &mut ResMut<SpatialIndex>,
    ) -> Entity {
        let team = match self.team {
            Team::Red => "red",
            Team::Blue => "blue",
        };
        let position = self.old_position;
        let unit = root
            .commands()
            .spawn(self)
            .with_children(|builder| {
                // builder.spawn(MeshBundle::new(
                //     "unit",
                //     "green_trans",
                //     vec4_to_trans(Vec4::new(0., 0., UNIT_RADIUS, 0.)),
                // ));
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
            .id();
        root.add_child(unit);
        spatial_index.add_unit(unit, PositionIndex::from_position(position));
        unit
    }
}

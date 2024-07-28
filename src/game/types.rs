use crate::game::graphics::MeshBundle;
use crate::game::{consts::*, utils::*};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

//================================================================================
// Generic Components ============================================================
//================================================================================

//attack override
#[derive(Component, PartialEq, Default, Copy, Clone, Debug)]
pub enum AttackOverride {
    #[default]
    Attack, //Unit will be distracted from current action to attack enemies within attack range
    Ignore, //Unit will not be distracted from current action
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
    Move(Vec2, AttackOverride),      //Unit moves to the location
    Attack(Entity, AttackBehaviour), //Unit moves within attack range of enemy
}
impl Default for Action {
    fn default() -> Self {
        Self::Stop(AttackOverride::Attack)
    }
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

#[derive(Resource, Default)]
pub struct SpatialIndex(HashMap<(i32, i32), HashSet<Entity>>);
impl SpatialIndex {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let tile = get_tile(pos);
        let mut nearby = Vec::new();
        for x in -1..=1 {
            for y in -1..=1 {
                if let Some(units) = self.0.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(units.iter());
                }
            }
        }
        nearby
    }
    pub fn add_unit(&mut self, entity: Entity, pos: Vec2) {
        let tile = get_tile(pos);
        if let Some(set) = self.0.get_mut(&tile) {
            set.insert(entity);
        } else {
            self.0.insert(tile, HashSet::from([entity]));
        }
    }
    pub fn remove_unit(&mut self, entity: Entity, pos: Vec2) {
        let tile = get_tile(pos);
        self.0.get_mut(&tile).unwrap().remove(&entity);
    }
    pub fn move_unit(&mut self, entity: Entity, old_pos: Vec2, new_pos: Vec2) {
        let old_tile = get_tile(old_pos);
        let new_tile = get_tile(new_pos);
        if old_tile != new_tile {
            self.0.get_mut(&old_tile).unwrap().remove(&entity);
            if let Some(set) = self.0.get_mut(&new_tile) {
                set.insert(entity);
            } else {
                self.0.insert(new_tile, HashSet::from([entity]));
            }
        }
    }
}
fn get_tile(pos: Vec2) -> (i32, i32) {
    let rounded = (pos / SPACIAL_INDEX_CELL_SIZE).round();
    (rounded.x as i32, rounded.y as i32)
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
    pub spatial: SpatialBundle,
    pub team: Team,
    pub lane: Lane,
    pub spawner: Spawner,
}
impl SpawnerBundle {
    pub fn new(vec2: Vec2, team: Team, lane: Lane) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(vec2.extend(0.).extend(0.))),
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
    pub spatial: SpatialBundle,
    //unit type
    pub team: Team,
    pub discipline: Discipline,
    //AI data
    pub action: Action,
    pub mid_crossed: MidCrossed,
    pub attack_timer: FixedTimer,
    //previous-frame data
    pub old_pos: OldPos,
    //tag for query filtering
    pub unit: Unit,
}
impl UnitBundle {
    pub fn new(vec2: Vec2, team: Team, discipline: Discipline) -> Self {
        Self {
            spatial: SpatialBundle::from_transform(vec4_to_trans(vec2.extend(0.).extend(0.))),
            team,
            discipline,
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
        unit
    }
}

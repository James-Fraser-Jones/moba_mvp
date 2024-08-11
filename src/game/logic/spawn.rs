use crate::game::*;
use bevy::prelude::*;
use logic::types::*;
use std::f32::consts::PI;
use std::sync::LazyLock;

//spawn settings
pub const CORE_SPAWN_POSITION: Vec2 = Vec2::splat(300.);
pub const SPAWNER_RELATIVE_SPAWN_RADIUS: f32 = 350.;
pub static TOPBOT_TOWER_RELATIVE_SPAWN_RADII: LazyLock<Vec<f32>> =
    LazyLock::new(|| vec![500., 830., 1350.]);
pub static MID_TOWER_RELATIVE_SPAWN_RADII: LazyLock<Vec<f32>> =
    LazyLock::new(|| vec![540., 780., 1000.]);
pub const ADVOCATE_SPAWN_NUM: i32 = 5;
pub const ADVOCATE_SPAWN_RING_POSITION: Vec2 = Vec2::splat(200.);
pub const ADVOCATE_SPAWN_RING_RADIUS: f32 = 50.;
pub const ADVOCATE_SPAWN_RING_ROTATION: f32 = PI / 2.;
pub const DEMON_SPAWN_POSITION: Vec2 = Vec2::new(1341., 586.);
pub static MONSTER_SPAWN_POSITIONS: LazyLock<Vec<Vec2>> = LazyLock::new(|| {
    vec![
        Vec2::new(1129., 351.),
        Vec2::new(1047., 527.),
        Vec2::new(949., 714.),
        Vec2::new(521., 878.),
        Vec2::new(512., 1066.),
        Vec2::new(293., 1129.),
    ]
});

//derived spawn consts/statics
pub const BASE_MID_CORNER: Vec2 = Vec2::splat(logic::OUTER_WALL_WIDTH + logic::LANE_WIDTH / 2.); //some spawns are relative to center-point of overlapping lanes in each base
pub static SPAWNER_SPAWN_POSITIONS: LazyLock<Vec<Vec2>> = LazyLock::new(|| {
    let mut positions = Vec::new();
    for lane in [Lane::Top, Lane::Mid, Lane::Bot] {
        let ang = PI / 4. * lane as i32 as f32;
        let point = Vec2::new(SPAWNER_RELATIVE_SPAWN_RADIUS, 0.).rotate(Vec2::from_angle(ang))
            + BASE_MID_CORNER;
        positions.push(point);
    }
    positions
});
pub static TOWER_SPAWN_POSITIONS: LazyLock<Vec<Vec2>> = LazyLock::new(|| {
    let mut positions = Vec::new();
    let zig_spacing = logic::LANE_WIDTH / 2. - logic::TOWER_RADIUS;
    for lane in [Lane::Top, Lane::Mid, Lane::Bot] {
        let ang = PI / 4. * lane as i32 as f32;
        let zig = lane != Lane::Bot;
        let points = if lane == Lane::Mid {
            &MID_TOWER_RELATIVE_SPAWN_RADII
        } else {
            &TOPBOT_TOWER_RELATIVE_SPAWN_RADII
        };
        for zig_point in zig_zag(points, zig, zig_spacing) {
            let point = zig_point.rotate(Vec2::from_angle(ang)) + BASE_MID_CORNER;
            positions.push(point);
        }
    }
    positions
});
pub static ADVOCATE_SPAWN_POSITIONS: LazyLock<Vec<Vec2>> = LazyLock::new(|| {
    let mut positions = Vec::new();
    for i in 0..ADVOCATE_SPAWN_NUM {
        let ang = ((2. * PI) / ADVOCATE_SPAWN_NUM as f32) * i as f32;
        let point = Vec2::new(ADVOCATE_SPAWN_RING_RADIUS, 0.)
            .rotate(Vec2::from_angle(ang + ADVOCATE_SPAWN_RING_ROTATION))
            + ADVOCATE_SPAWN_RING_POSITION;
        positions.push(point);
    }
    positions
});

//tower spawning utility
fn zig_zag(points: &Vec<f32>, zig_first: bool, zig_spacing: f32) -> Vec<Vec2> {
    let mut zig_zag = Vec::new();
    for (i, point) in points.iter().enumerate() {
        zig_zag.push(Vec2::new(
            *point,
            if (i % 2 == 0) == zig_first { 1. } else { -1. } * zig_spacing,
        ));
    }
    zig_zag
}

pub fn spawn_everything(commands: &mut Commands) {
    for team in [Team::Red, Team::Blue] {
        commands.spawn(Core::new(
            logic::reframe_position(CORE_SPAWN_POSITION, team, true),
            team,
        ));
        for position in SPAWNER_SPAWN_POSITIONS.iter() {
            commands.spawn(Spawner::new(
                logic::reframe_position(*position, team, true),
                team,
            ));
        }
        for position in TOWER_SPAWN_POSITIONS.iter() {
            commands.spawn(Tower::new(
                logic::reframe_position(*position, team, true),
                team,
            ));
        }
        for position in ADVOCATE_SPAWN_POSITIONS.iter() {
            commands.spawn(Advocate::new(
                logic::reframe_position(*position, team, true),
                team,
            ));
        }
        for position in MONSTER_SPAWN_POSITIONS.iter() {
            commands.spawn(Monster::new(logic::reframe_position(*position, team, true)));
        }
        commands.spawn(Demon::new(logic::reframe_position(
            DEMON_SPAWN_POSITION,
            team,
            true,
        )));
    }
}

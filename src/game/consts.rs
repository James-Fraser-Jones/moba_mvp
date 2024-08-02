use bevy::prelude::*;

//unit
pub const UNIT_SPEED: f32 = 185.;
pub const UNIT_RADIUS: f32 = 18.5;
pub const UNIT_SIGHT_RADIUS: f32 = 200.;
pub const UNIT_ATTACK_RADIUS: f32 = 37.;
pub const UNIT_ATTACK_SPEED: f32 = 1.;
pub const UNIT_HEALTH: f32 = 100.;
pub const UNIT_ATTACK_DAMAGE: f32 = 10.;

//map
pub const LANE_WIDTH: f32 = 240.;

//locations
pub const NON_LANE_RADIUS: f32 = 1000. - LANE_WIDTH;
pub const MID_LANE_RADIUS: f32 = 1000. - LANE_WIDTH / 2.;
pub const TOP: Vec2 = Vec2::new(-MID_LANE_RADIUS, MID_LANE_RADIUS);
pub const RED_TOP: Vec2 = Vec2::new(-MID_LANE_RADIUS, 0.);
pub const BLUE_TOP: Vec2 = Vec2::new(0., MID_LANE_RADIUS);
pub const MID: Vec2 = Vec2::new(0., 0.);
pub const BOT: Vec2 = Vec2::new(MID_LANE_RADIUS, -MID_LANE_RADIUS);
pub const RED_BOT: Vec2 = Vec2::new(0., -MID_LANE_RADIUS);
pub const BLUE_BOT: Vec2 = Vec2::new(MID_LANE_RADIUS, 0.);
pub const RED: Vec2 = Vec2::new(-MID_LANE_RADIUS, -MID_LANE_RADIUS);
pub const BLUE: Vec2 = Vec2::new(MID_LANE_RADIUS, MID_LANE_RADIUS);

//waves
pub const WAVE_DELAY: f32 = 20.;
pub const WAVE_NUM_UNITS: i32 = 6;

//spawners
pub const SPAWNER_DELAY: f32 = 1.;

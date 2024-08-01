use bevy::{
    prelude::*,
    window::{CursorGrabMode, WindowMode},
};
use std::f32::consts::PI;

//camera
pub const CAMERA_SPEED: f32 = 800.;
pub const CAMERA_TURN_SPEED: f32 = PI / 20.;
pub const CAMERA_FOV: f32 = PI / 4.; //this is the default
pub const CAMERA_FOV_SCALE: f32 = 2. / 3.; //how much to multiplicatively increase/decrease on-zoom
pub const CAMERA_ORTHOGRAPHIC: bool = true;
pub const CAMERA_FAR: f32 = 10000.;

//unit
pub const UNIT_SPEED: f32 = 185.;
pub const UNIT_RADIUS: f32 = 18.5;
pub const UNIT_SIGHT_RADIUS: f32 = 200.;
pub const UNIT_ATTACK_RADIUS: f32 = 37.;
pub const UNIT_ATTACK_SPEED: f32 = 1.;
pub const UNIT_HEALTH: f32 = 100.;
pub const UNIT_ATTACK_DAMAGE: f32 = 10.;
pub const UNIT_TRIANGLE_ANGLE: f32 = PI / 8.;

//window
pub const WINDOW_NAME: &str = "Moba MVP";
pub const WINDOW_POSITION: IVec2 = IVec2::new(0, 0);
pub const WINDOW_SIZE: Vec2 = Vec2::new(960., 540.);
pub const WINDOW_ASPECT_RATIO: f32 = WINDOW_SIZE.x / WINDOW_SIZE.y;
//pub const WINDOW_MODE: WindowMode = WindowMode::BorderlessFullscreen;
pub const WINDOW_MODE: WindowMode = WindowMode::Windowed;
//pub const WINDOW_CURSOR_GRAB_MODE: CursorGrabMode = CursorGrabMode::Confined;
pub const WINDOW_CURSOR_GRAB_MODE: CursorGrabMode = CursorGrabMode::None;

//materials
pub const SATURATION: f32 = 0.75;
pub const BRIGHTNESS: f32 = 0.5;
pub const RED_HUE: f32 = 0.;
pub const GREEN_HUE: f32 = 120.;
pub const BLUE_HUE: f32 = 240.;
pub const TEAL_HUE: f32 = 190.;
pub const YELLOW_HUE: f32 = 60.;
pub const PURPLE_HUE: f32 = 275.;
pub const UNLIT: bool = true;

//map
pub const LANE_WIDTH: f32 = 240.;
pub const RIVER_WIDTH: f32 = 200.;
pub const BASE_RADIUS: f32 = 360.;

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
pub const SPAWNER_RADIUS: f32 = 27.8;
pub const SPAWNER_DELAY: f32 = 1.;

//spacial index
pub const CELL_HALF_SIZE: f32 = 25.;

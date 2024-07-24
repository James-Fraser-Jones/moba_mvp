use std::f32::consts::PI;

//camera
pub const CAMERA_SPEED: f32 = 500.;
pub const ZOOM_SPEED: f32 = 0.1;

//unit
pub const UNIT_SPEED: f32 = 100.;
pub const UNIT_RADIUS: f32 = 10.; //if set to factor of GCD of SCREEN_WIDTH and SCREEN_HEIGHT, can have a grid with square cells that fits the screen perfectly (currently: 120)
pub const UNIT_TRIANGLE_ANGLE: f32 = PI / 4.;

//window
pub const APP_NAME: &str = "Moba MVP";
pub const SCREEN_WIDTH: f32 = 1920.;
pub const SCREEN_HEIGHT: f32 = 1080.;

//colors
pub const SATURATION: f32 = 0.75;
pub const BRIGHTNESS: f32 = 0.5;
pub const RED_HUE: f32 = 0.;
pub const GREEN_HUE: f32 = 120.;
pub const BLUE_HUE: f32 = 240.;
pub const TEAL_HUE: f32 = 190.;
pub const YELLOW_HUE: f32 = 60.;
pub const PURPLE_HUE: f32 = 275.;

//map
pub const MAP_SIZE: f32 = SCREEN_HEIGHT;

pub const LANE_WIDTH: f32 = 0.12;
pub const RIVER_WIDTH: f32 = 0.1;
pub const BASE_RADIUS: f32 = 0.18;

pub const MID_LANE: f32 = (1. - LANE_WIDTH) / 2.;
pub const NON_LANE_WIDTH: f32 = 1. - 2. * LANE_WIDTH;

//waves
pub const WAVE_DELAY: f32 = 20.;
pub const WAVE_NUM_UNITS: i32 = 6;

//spawners
pub const SPAWNER_RADIUS: f32 = 15.;
pub const SPAWNER_POS_RADIUS: f32 = 0.15;
pub const SPAWNER_DELAY: f32 = 1.;

//directions
pub const NORTH: f32 = 2. * PI / 4.;
pub const NORTH_EAST: f32 = 1. * PI / 4.;
pub const EAST: f32 = 0. * PI / 4.;
pub const SOUTH_EAST: f32 = 7. * PI / 4.;
pub const SOUTH: f32 = 6. * PI / 4.;
pub const SOUTH_WEST: f32 = 5. * PI / 4.;
pub const WEST: f32 = 4. * PI / 4.;
pub const NORTH_WEST: f32 = 3. * PI / 4.;

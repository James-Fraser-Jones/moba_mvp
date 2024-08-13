//responsibilities:
//drive the main logic of the game
//no consideration of anything other than core logic
//necessarily must perform physics calculations since they dictate discrete logic such as whether a skillshot landed, etc..
//entirely self contained, no direct communication with any other plugins
//can recieve events from player plugins to determine player actions, through a strongly-typed interface (only source of non-determinism)

pub mod spawn;
pub mod types;

use super::*;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::sync::LazyLock;
use types::*;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

const TOWER_RADIUS: f32 = 20.;

//descriptive of the blender file
const LANE_WIDTH: f32 = 130.;
const OUTER_WALL_WIDTH: f32 = 100.;

//multiply Vec3s in X_TRANSFORM space, by X_TRANSFORM, to convert them to GLOBAL space
pub static RED_TRANSFORM: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_translation(Vec2::splat(-1000.).extend(0.)));
pub static BLUE_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| {
    Transform::from_translation(Vec2::splat(1000.).extend(0.))
        .with_rotation(Quat::from_rotation_z(PI))
});
//multiply Vec3s in GLOBAL space, by X_TRANSFORM_INVERSE, to convert them to X_TRANSFORM space
pub static RED_TRANSFORM_INVERSE: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_matrix(RED_TRANSFORM.compute_matrix().inverse()));
pub static BLUE_TRANSFORM_INVERSE: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_matrix(BLUE_TRANSFORM.compute_matrix().inverse()));
fn team_transform(team: Team, inverse: bool) -> Transform {
    match team {
        Team::Red => {
            if inverse {
                *RED_TRANSFORM_INVERSE
            } else {
                *RED_TRANSFORM
            }
        }
        Team::Blue => {
            if inverse {
                *BLUE_TRANSFORM_INVERSE
            } else {
                *BLUE_TRANSFORM
            }
        }
    }
}
fn reframe_position(position: Vec2, team: Team, to_global: bool) -> Vec2 {
    (team_transform(team, !to_global) * position.extend(0.)).truncate()
}

fn init(mut commands: Commands) {
    spawn::spawn_everything(&mut commands);
}

fn update() {}

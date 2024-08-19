//responsibilities:
//drive the main logic of the game
//no consideration of anything other than core logic
//necessarily must perform physics calculations since they dictate discrete logic such as whether a skillshot landed, etc..
//entirely self contained, no direct communication with any other plugins
//can recieve events from player plugins to determine player actions, through a strongly-typed interface (only source of non-determinism)

use crate::game::*;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::sync::LazyLock;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(TICK_RATE as f64));
        app.add_systems(Startup, init);
        app.add_systems(FixedUpdate, update_move.in_set(LogicSet));
    }
}

const TICK_RATE: f32 = 16.0;
//multiply Vec3s in X_TRANSFORM space, by X_TRANSFORM, to convert them to GLOBAL space
static RED_TRANSFORM: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_translation(Vec2::splat(-1000.).extend(0.)));
static BLUE_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| {
    Transform::from_translation(Vec2::splat(1000.).extend(0.))
        .with_rotation(Quat::from_rotation_z(PI))
});
//multiply Vec3s in GLOBAL space, by X_TRANSFORM_INVERSE, to convert them to X_TRANSFORM space
static RED_TRANSFORM_INVERSE: LazyLock<Transform> =
    LazyLock::new(|| Transform::from_matrix(RED_TRANSFORM.compute_matrix().inverse()));
static BLUE_TRANSFORM_INVERSE: LazyLock<Transform> =
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
pub fn reframe_position(position: Vec2, team: Team, to_global: bool) -> Vec2 {
    (team_transform(team, !to_global) * position.extend(0.)).truncate()
}

fn init(mut commands: Commands) {
    commands.spawn(Minion::new(Vec2::ZERO, Team::Red));
}

fn update_move(mut query: Query<(&mut Transform, &mut MovePosition, &MoveSpeed)>, time: Res<Time>) {
    for (mut transform, mut move_position, move_speed) in &mut query {
        if let Some(goal) = move_position.0 {
            let pos = transform.translation.truncate();
            let diff = goal - pos;
            let new_pos = pos + diff.clamp_length_max(move_speed.0 * time.delta_seconds());

            transform.translation = new_pos.extend(0.);
            transform.rotation = Quat::from_rotation_z(diff.to_angle());

            if new_pos == goal {
                move_position.0 = None;
            }
        }
    }
}

use crate::lib::{consts::*, types::*, utils::*};
use bevy::prelude::*;

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (spawn_units, move_units, collide_units).chain(),
        );
    }
}

fn spawn_units(
    mut query: Query<(&Transform, &Team, &mut SpawnTimer), With<Spawner>>,
    mut commands: Commands,
    handles: Res<Handles>,
    time: Res<Time>,
) {
    for (transform, team, mut spawn_timer) in &mut query {
        spawn_timer.0.tick(time.delta());
        if spawn_timer.0.finished() {
            let unit = UnitBundle::from_xyrt(
                transform.translation.x,
                transform.translation.y,
                transform.rotation.to_axis_angle().1,
                *team,
            );
            spawn_unit(&mut commands, &handles, unit);
        }
    }
}

fn move_units(mut query: Query<&mut Transform, With<Unit>>, time: Res<Time>) {
    for mut transform in &mut query {
        let direction = transform.local_x().as_vec3();
        transform.translation += direction * UNIT_SPEED * time.delta_seconds();
    }
}

fn collide_units(mut query: Query<&mut Transform, With<Unit>>) {
    let mut transforms = query.iter_combinations_mut(); //combinations don't include pairs of refs to a single entity
    while let Some([mut transform_a, mut transform_b]) = transforms.fetch_next() {
        let mut pos_a = transform_a.translation.truncate();
        let mut pos_b = transform_b.translation.truncate();
        let a_to_b = pos_b - pos_a;
        let collide_dist = 2. * UNIT_RADIUS - a_to_b.length();
        if collide_dist > 0. {
            let a_to_b_dir = a_to_b.normalize();
            pos_a -= a_to_b_dir * collide_dist / 2.;
            pos_b += a_to_b_dir * collide_dist / 2.;
            transform_a.translation = pos_a.extend(transform_a.translation.z);
            transform_b.translation = pos_b.extend(transform_b.translation.z);
        }
    }
}

use crate::helpers::{consts::*, types::*, utils::*};
use bevy::prelude::*;

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_timers, manage_waves, move_units, collide_units).chain(),
        );
    }
}

fn update_timers(mut query: Query<&mut FixedTimer>, time: Res<Time>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}

fn manage_waves(
    mut wave_manager: ResMut<WaveManager>,
    spawner_query: Query<(&Transform, &Team), With<Spawner>>,
    mut commands: Commands,
    handles: Res<Handles>,
    time: Res<Time>,
) {
    //advance time
    wave_manager.wave_timer.tick(time.delta());
    wave_manager.spawn_timer.tick(time.delta()); //does nothing if paused

    if !wave_manager.spawn_timer.paused() {
        //if we are currently spawning a wave
        if wave_manager.spawn_index < WAVE_NUM_UNITS {
            //if we have not reached the end of this wave
            if wave_manager.spawn_timer.finished() {
                //spawn a unit at each spawner
                for (transform, team) in &spawner_query {
                    let mut vec4 = trans_to_vec4(transform);
                    vec4.z = 0.; //reset z-index;
                    let unit = UnitBundle::new(vec4, *team, Discipline::Melee, Lane::Mid);
                    spawn_unit(&mut commands, &handles, unit);
                }
                wave_manager.spawn_index += 1;
            }
        } else {
            //if we have reached the end of this wave
            wave_manager.spawn_timer.reset();
            wave_manager.spawn_timer.pause();
            wave_manager.spawn_index = 0;
        }
    } else {
        //if we are not spawning a wave
        if wave_manager.wave_timer.finished() {
            wave_manager.spawn_timer.unpause();
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

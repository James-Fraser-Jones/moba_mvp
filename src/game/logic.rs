use crate::game::{consts::*, types::*};
use avian2d::prelude::*;
use bevy::prelude::*;
pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::new(FixedUpdate));
        app.add_systems(Startup, init_map);
        app.add_systems(
            FixedUpdate,
            (
                update_timers,
                spawn_units,
                units_decide_action,
                units_execute_action,
                update_orientations,
            )
                .chain()
                .before(PhysicsSet::Prepare),
        );
    }
}

fn init_map(mut commands: Commands) {
    //add resources
    commands.insert_resource(WaveManager::new());
    //add entities
    MapBundle::new().spawn(&mut commands);
    for (lane_pos, lane) in [(TOP, Lane::Top), (MID, Lane::Mid), (BOT, Lane::Bot)] {
        for (team_pos, team) in [(RED, Team::Red), (BLUE, Team::Blue)] {
            let diff = (lane_pos - team_pos).normalize();
            let pos = Pos(team_pos + diff * (BASE_RADIUS - LANE_WIDTH));
            SpawnerBundle::new(pos, team, lane).spawn(&mut commands);
        }
    }
}

fn update_timers(mut query: Query<&mut FixedTimer>, time: Res<Time>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}

fn spawn_units(
    mut wave_manager: ResMut<WaveManager>,
    spawner_query: Query<(&Transform, &Team, &Lane), With<Spawner>>,
    mut commands: Commands,
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
                for (transform, team, lane) in &spawner_query {
                    let pos = Pos::from_transform(transform);
                    let destination = Pos(match *lane {
                        Lane::Top => TOP,
                        Lane::Mid => MID,
                        Lane::Bot => BOT,
                    });
                    UnitBundle::new(
                        pos,
                        *team,
                        Discipline::Melee,
                        Action::Move(destination, AttackOverride::Attack),
                    )
                    .spawn(&mut commands);
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

fn units_decide_action(mut query: Query<(&mut Action, &Transform, &Team, &mut MidCrossed)>) {
    for (mut action, trans, team, mut mid_crossed) in &mut query {
        match *action {
            Action::Stop(_attack) => {} //(selective) vision radius collision check with enemies
            Action::Move(dest, attack) => {
                if Pos::from_transform(trans).0.distance(dest.0) < UNIT_RADIUS {
                    if mid_crossed.0 {
                        *action = Action::Stop(attack);
                    } else {
                        mid_crossed.0 = true;
                        *action = Action::Move(
                            Pos(match *team {
                                Team::Red => BLUE,
                                Team::Blue => RED,
                            }),
                            attack,
                        )
                    }
                }
            } //(selective) vision radius collision check with enemies, unit radius (collision?) check with midpoint of lane
            Action::Attack(_target, behaviour) => match behaviour {
                AttackBehaviour::Pursue => {} //possibly go into attack mode
                AttackBehaviour::Attack => {} //possible go into pursuit, canceling the attack timer
            }, //attack range collision check with target
        }
    }
}

//technically we want to borrow:
//unit translation (immutably)
//unit rotation (mutably)
//target translation (immutably)
//which shouldn't be an issue except we can't query for something smaller than a component
fn units_execute_action(
    mut query: Query<(&Action, &Transform, &mut LinearVelocity), With<Unit>>,
    target_query: Query<&Transform, With<Unit>>,
) {
    for (action, trans, mut linear_velocity) in &mut query {
        //do not move, by default
        *linear_velocity = LinearVelocity(Vec2::ZERO);
        match *action {
            Action::Stop(_) => {} //do nothing
            Action::Move(dest, _) => {
                move_unit(dest, trans, &mut linear_velocity);
            }
            Action::Attack(target, behaviour) => match behaviour {
                AttackBehaviour::Pursue => {
                    let dest = Pos::from_transform(target_query.get(target).unwrap());
                    move_unit(dest, trans, &mut linear_velocity);
                }
                AttackBehaviour::Attack => {} //implement attack timer logic
            },
        }
    }
}

fn move_unit(dest: Pos, trans: &Transform, linear_velocity: &mut LinearVelocity) {
    let to = dest.0 - Pos::from_transform(&trans).0;
    *linear_velocity = LinearVelocity(to.normalize_or_zero() * UNIT_SPEED);
}

fn update_orientations(mut query: Query<(&mut Transform, &LinearVelocity), With<Unit>>) {
    for (mut trans, linear_velocity) in &mut query {
        trans.rotation = Quat::from_rotation_z(linear_velocity.0.to_angle());
    }
}

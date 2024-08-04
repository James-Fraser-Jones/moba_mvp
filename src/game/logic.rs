mod types;

use avian2d::prelude::*;
use bevy::prelude::*;

pub struct LogicPlugin;
impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::new(FixedUpdate));
        app.add_systems(Startup, init);
        app.add_systems(
            FixedUpdate,
            (update_waves, update_units_decision, update_units_execution)
                .chain()
                .before(PhysicsSet::Prepare),
        );
    }
}

fn init(mut commands: Commands) {
    let map_settings = MapSettings::default();
    let wave_settings = WaveSettings::default();

    //add entities
    for (lane_pos, lane) in [(TOP, Lane::Top), (MID, Lane::Mid), (BOT, Lane::Bot)] {
        for (team_pos, team) in [(RED, Team::Red), (BLUE, Team::Blue)] {
            let diff = (lane_pos - team_pos).normalize();
            let pos = Pos(team_pos + diff * (map_settings.base_radius - LANE_WIDTH));
            SpawnerBundle::new(pos, team, lane).spawn(&mut commands);
        }
    }

    //add resources
    commands.insert_resource(WaveManager::new(
        wave_settings.wave_delay,
        wave_settings.spawn_delay,
        wave_settings.spawn_count,
    ));
}

fn update_waves(
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
                    commands.spawn(UnitBundle::new(
                        pos,
                        *team,
                        Discipline::Melee,
                        Action::Move(destination, AttackOverride::Attack),
                    ));
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

fn update_units_decision(
    sight_query: Query<(&CollidingEntities, &Parent), With<SightCollider>>,
    mut unit_query: Query<&mut Action, With<Unit>>,
) {
    for (in_vision, unit) in &sight_query {
        let mut action = unit_query.get_mut(unit.get()).unwrap();
        match *action {
            Action::Stop(attack) | Action::Move(_, attack) => {
                if attack == AttackOverride::Attack {
                    if let Some(target) = in_vision.0.iter().next() {
                        *action = Action::Attack(*target, AttackBehaviour::Pursue);
                    }
                }
            }
            _ => (),
        }
    }
    for (mut unit, trans, team) in &mut query {
        if let Action::Move(dest, attack) = unit.action {
            if Pos::from_transform(trans).0.distance(dest.0) < UNIT_RADIUS {
                if unit.mid_crossed {
                    unit.action = Action::Stop(attack);
                } else {
                    unit.action = Action::Move(
                        Pos(match *team {
                            Team::Red => BLUE,
                            Team::Blue => RED,
                        }),
                        attack,
                    );
                    unit.mid_crossed = true;
                }
            }
        }
    }
}

fn update_units_execution(
    mut query: Query<(&Action, &Transform, &mut LinearVelocity), With<Unit>>,
    target_query: Query<&Transform, With<Unit>>,
) {
    for (action, trans, mut linear_velocity) in &mut query {
        *linear_velocity = LinearVelocity(Vec2::ZERO); //do not move, by default
        match *action {
            Action::Move(dest, _) => {
                move_unit(dest, trans, &mut linear_velocity);
            }
            Action::Attack(target, behaviour) => match behaviour {
                AttackBehaviour::Pursue => {
                    let dest = Pos::from_transform(target_query.get(target).unwrap());
                    move_unit(dest, trans, &mut linear_velocity);
                }
                AttackBehaviour::Attack => {} //TODO: implement attack timer logic
            },
            _ => (),
        }
    }
}

fn move_unit(dest: Pos, trans: &Transform, linear_velocity: &mut LinearVelocity) {
    let to = dest.0 - Pos::from_transform(&trans).0;
    *linear_velocity = LinearVelocity(to.normalize_or_zero() * UNIT_SPEED);
}

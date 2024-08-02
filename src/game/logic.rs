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
                units_check_ranges,
                units_update_path,
                units_execute_action,
                update_orientations,
            )
                .chain()
                .before(PhysicsSet::Prepare),
        );
    }
}

#[derive(Resource)]
pub struct LogicMapSettings {
    lane_width: f32,
    wave_delay: f32,
    wave_units: i32,
    spawn_delay: f32,
}
impl Default for LogicMapSettings {
    fn default() -> Self {
        Self {
            lane_width: 240.,
            wave_delay: 20.,
            wave_units: 6,
            spawn_delay: 1.,
        }
    }
}
impl LogicMapSettings {
    fn non_lane_radius(&self) -> f32 {
        1000. - self.lane_width
    }
    fn mid_lane_radius(&self) -> f32 {
        1000. - self.lane_width / 2.
    }
    fn location(&self, lane: Lane, team: Team) -> Vec2 {
        Vec2::ZERO
    }
}

#[derive(Resource)]
pub struct LogicUnitSettings {
    radius: f32,
    speed: f32,
    sight_radius: f32,
    attack_radius: f32,
    attack_speed: f32,
    health: f32,
    attack_damage: f32,
}
impl Default for LogicUnitSettings {
    fn default() -> Self {
        Self {
            radius: 18.5,
            speed: 185.,
            sight_radius: 55.6,
            attack_radius: 37.,
            attack_speed: 1.,
            health: 100.,
            attack_damage: 10.,
        }
    }
}

#[derive(Resource)]
pub struct LogicSpawnerSettings;

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

fn units_check_ranges(
    sight_query: Query<(&CollidingEntities, &Parent), With<SightCollider>>,
    attack_query: Query<(&CollidingEntities, &Parent), With<AttackCollider>>,
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
    for (in_attack_range, unit) in &sight_query {
        let mut action = unit_query.get_mut(unit.get()).unwrap();
        match *action {
            Action::Attack(_target, behaviour) => match behaviour {
                //TODO: collision check on attack radius
                AttackBehaviour::Pursue => {}
                AttackBehaviour::Attack => {} //aa timer can be cancelled here
            },
            _ => (),
        }
    }
}

fn units_update_path(
    mut query: Query<(&mut Action, &Transform, &Team, &mut MidCrossed), With<Unit>>,
) {
    for (mut action, trans, team, mut mid_crossed) in &mut query {
        if let Action::Move(dest, attack) = *action {
            if Pos::from_transform(trans).0.distance(dest.0) < UNIT_RADIUS {
                if mid_crossed.0 {
                    *action = Action::Stop(attack);
                } else {
                    *action = Action::Move(
                        Pos(match *team {
                            Team::Red => BLUE,
                            Team::Blue => RED,
                        }),
                        attack,
                    );
                    mid_crossed.0 = true;
                }
            }
        }
    }
}

fn units_execute_action(
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

fn update_orientations(mut query: Query<(&Action, &mut Transform, &LinearVelocity), With<Unit>>) {
    for (action, mut trans, linear_velocity) in &mut query {
        if let Action::Move(_, _) = *action {
            trans.rotation = Quat::from_rotation_z(linear_velocity.0.to_angle());
        } else if let Action::Attack(_, AttackBehaviour::Pursue) = *action {
            trans.rotation = Quat::from_rotation_z(linear_velocity.0.to_angle());
        }
    }
}

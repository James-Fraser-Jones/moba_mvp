use crate::game::{consts::*, types::*, utils::*};
use bevy::prelude::*;
use rand::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_map);
        app.add_systems(
            FixedUpdate,
            (
                update_timers,
                spawn_units,
                // units_decide_action,
                // units_execute_action,
                // update_positions,
            )
                .chain(),
        );
    }
}

fn init_map(mut commands: Commands) {
    //add resources
    commands.insert_resource(WaveManager::new());
    commands.insert_resource(SpatialIndex::new());
    //add entities
    let root_id = RootBundle::new().spawn(&mut commands);
    if let Some(mut root) = commands.get_entity(root_id) {
        //map
        MapBundle::new().spawn(&mut root);
        //spawners
        for (lane_pos, lane) in [(TOP, Lane::Top), (MID, Lane::Mid), (BOT, Lane::Bot)] {
            for (team_pos, team) in [(RED, Team::Red), (BLUE, Team::Blue)] {
                let diff = (lane_pos - team_pos).normalize();
                let position = Position(team_pos + diff * (BASE_RADIUS - LANE_WIDTH));
                SpawnerBundle::new(position, team, lane).spawn(&mut root);
            }
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
    root_query: Query<Entity, With<Root>>,
    mut spatial_index: ResMut<SpatialIndex>,
) {
    let root_id = root_query.single();
    if let Some(mut root) = commands.get_entity(root_id) {
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
                        let position = Position::from_transform(transform);
                        let destination = Position(match *lane {
                            Lane::Top => TOP,
                            Lane::Mid => MID,
                            Lane::Bot => BOT,
                        });
                        UnitBundle::new(
                            position,
                            *team,
                            Discipline::Melee,
                            Action::Move(destination, AttackOverride::Attack),
                        )
                        .spawn(&mut root, &mut spatial_index);
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
}

fn check_for_attack_target(
    team: Team,
    position: Position,
    target_query: &Query<(&Team, &Position)>,
    spatial_index: &Res<SpatialIndex>,
) -> Option<Entity> {
    let sight_radius = Radius(UNIT_SIGHT_RADIUS + MAX_UNIT_RADIUS);
    for target in
        spatial_index.get_nearby_units(PositionIndex::from_position(position), sight_radius)
    {
        let (target_team, target_position) = target_query.get(target).unwrap();
        if team != *target_team {
            if position.0.distance(target_position.0) <= sight_radius.0 {
                return Some(target); //this simply selects the first unit found, which may technically not be the closest
            }
        }
    }
    return None;
}

fn set_attack_behaviour(
    mut action: Mut<Action>,
    pos_query: &Query<(&Team, &Position)>,
    target: Entity,
    position: Position,
) {
    //TODO: more complex logic accounting for the fact that timers (sometimes) need to be reset and set to different durations
    //depending on the flip from Attack->Pursue and vice versa
    let attack_radius = Radius(UNIT_ATTACK_RADIUS + MAX_UNIT_RADIUS);
    let (_, target_position) = pos_query.get(target).unwrap();
    *action = Action::Attack(
        target,
        if position.0.distance(target_position.0) <= attack_radius.0 {
            AttackBehaviour::Attack
        } else {
            AttackBehaviour::Pursue
        },
    );
}

fn units_decide_action(
    mut query: Query<(&Transform, &mut Action, &Team, &mut MidCrossed)>,
    position_query: Query<(&Team, &Position)>,
    spatial_index: Res<SpatialIndex>,
) {
    for (transform, mut action, team, mut mid_crossed) in &mut query {
        let position = Position(transform.translation.truncate());
        match *action {
            Action::Stop(attack) => {
                if attack == AttackOverride::Attack {
                    if let Some(target) =
                        check_for_attack_target(*team, position, &position_query, &spatial_index)
                    {
                        set_attack_behaviour(action, &position_query, target, position);
                    }
                }
            }
            Action::Move(destination, attack) => {
                if attack == AttackOverride::Attack {
                    if let Some(target) =
                        check_for_attack_target(*team, position, &position_query, &spatial_index)
                    {
                        set_attack_behaviour(action, &position_query, target, position);
                    }
                } else {
                    let move_success_radius = Radius(UNIT_RADIUS);
                    if position.0.distance(destination.0) <= move_success_radius.0 {
                        if mid_crossed.0 {
                            *action = Action::Stop(attack);
                        } else {
                            mid_crossed.0 = true;
                            let new_destination = Position(match *team {
                                Team::Red => BLUE,
                                Team::Blue => RED,
                            });
                            *action = Action::Move(new_destination, attack);
                        }
                    }
                }
            }
            Action::Attack(target, _) => {
                set_attack_behaviour(action, &position_query, target, position)
            }
        }
    }
}

fn move_unit(
    position: Position,
    destination: Position,
    time: &Res<Time>,
    mut transform: Mut<Transform>,
) {
    let wriggle = Vec2::new(
        thread_rng().gen_range(-1.0..=1.0),
        thread_rng().gen_range(-1.0..=1.0),
    ) * UNIT_WRIGGLE;
    let direction = (destination.0 - position.0 + wriggle).normalize(); //TODO: this could cause units to oscillate back and forth
    transform.translation += direction.extend(0.) * UNIT_SPEED * time.delta_seconds();
    transform.rotation = Quat::from_rotation_z(direction.to_angle());
    //TODO: collision avoidance logic
}

fn units_execute_action(
    mut query: Query<(&mut Transform, &mut Action, &Lane, &Team, &mut MidCrossed)>,
    position_query: Query<&Position>,
    time: Res<Time>,
) {
    for (mut transform, mut action, lane, team, mut mid_crossed) in &mut query {
        let position = Position(transform.translation.truncate());
        match *action {
            Action::Stop(attack) => {}
            Action::Move(destination, attack) => {
                move_unit(position, destination, &time, transform);
            }
            Action::Attack(target, behaviour) => match behaviour {
                AttackBehaviour::Pursue => {
                    let destination = position_query.get(target).unwrap();
                    move_unit(position, *destination, &time, transform);
                }
                AttackBehaviour::Attack => {
                    //TODO: attack logic
                }
            },
        }
    }
}

fn update_positions(mut query: Query<(&Transform, &mut Position), With<Unit>>) {
    for (transform, mut pos) in &mut query {
        *pos = Position(trans_to_vec4(transform).truncate().truncate());
    }
}

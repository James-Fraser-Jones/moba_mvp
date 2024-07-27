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
                manage_waves,
                unit_ai,
                move_units,
                check_other_units,
                update_old_pos,
            )
                .chain(),
        );
    }
}

fn init_map(mut commands: Commands) {
    //add resources
    commands.insert_resource(WaveManager::new());
    //add entities
    let root_id = RootBundle::new().spawn(&mut commands);
    if let Some(mut root) = commands.get_entity(root_id) {
        //map
        MapBundle::new().spawn(&mut root);
        //spawners
        for (lane_pos, lane) in [(TOP, Lane::Top), (MID, Lane::Mid), (BOT, Lane::Bot)] {
            for (team_pos, team) in [(RED, Team::Red), (BLUE, Team::Blue)] {
                let diff = (lane_pos - team_pos).normalize();
                let vec2 = team_pos + diff * (BASE_RADIUS - LANE_WIDTH);
                SpawnerBundle::new(vec2, team, lane).spawn(&mut root);
            }
        }
        //UnitBundle::new(Vec2::ZERO, Team::Red, Discipline::Melee, Lane::Top).spawn(&mut root);
    }
}

fn update_timers(mut query: Query<&mut FixedTimer>, time: Res<Time>) {
    for mut timer in &mut query {
        timer.0.tick(time.delta());
    }
}

fn manage_waves(
    mut wave_manager: ResMut<WaveManager>,
    spawner_query: Query<(&Transform, &Team, &Lane), With<Spawner>>,
    mut commands: Commands,
    time: Res<Time>,
    root_query: Query<Entity, With<Root>>,
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
                        let vec2 = trans_to_vec4(transform).truncate().truncate();
                        UnitBundle::new(vec2, *team, Discipline::Melee, *lane).spawn(&mut root);
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

fn unit_ai(mut query: Query<(&Transform, &mut MoveType, &Lane, &Team, &mut MidCrossed)>) {
    for (transform, mut move_type, lane, team, mut mid_crossed) in &mut query {
        match *move_type {
            MoveType::Stationary => {
                *move_type = MoveType::AttackMove(if mid_crossed.0 {
                    match *team {
                        Team::Red => BLUE,
                        Team::Blue => RED,
                    }
                } else {
                    match *lane {
                        Lane::Bot => BOT,
                        Lane::Mid => MID,
                        Lane::Top => TOP,
                    }
                });
            }
            MoveType::AttackMove(pos) => {
                if (pos - transform.translation.truncate()).length() < UNIT_LOCATION_RADIUS {
                    mid_crossed.0 = true;
                    *move_type = MoveType::Stationary;
                }
            }
            MoveType::Move(_) | MoveType::Attack(_) => {}
        }
    }
}

fn move_units(
    mut query: Query<(&mut Transform, &mut MoveType), With<Unit>>,
    old_pos_query: Query<&OldPos>,
    time: Res<Time>,
) {
    for (mut transform, move_type) in &mut query {
        let pos = match *move_type {
            MoveType::Stationary => None,
            MoveType::Move(pos) | MoveType::AttackMove(pos) => Some(pos),
            MoveType::Attack(target) => Some(old_pos_query.get(target).unwrap().0),
        };
        if let Some(pos) = pos {
            let wriggle = Vec2::new(
                thread_rng().gen_range(-1.0..=1.0),
                thread_rng().gen_range(-1.0..=1.0),
            ) * UNIT_WRIGGLE;
            let direction = (pos - transform.translation.truncate() + wriggle).normalize();
            transform.translation += direction.extend(0.) * UNIT_SPEED * time.delta_seconds();
            transform.rotation = Quat::from_rotation_z(direction.to_angle());
        }
    }
}

fn check_other_units(mut query: Query<(Entity, &mut Transform, &mut MoveType), With<Unit>>) {
    let mut transforms = query.iter_combinations_mut(); //combinations don't include pairs of refs to a single entity
    while let Some(
        [(self_ref_a, mut transform_a, mut move_type_a), (self_ref_b, mut transform_b, mut move_type_b)],
    ) = transforms.fetch_next()
    {
        let mut pos_a = transform_a.translation.truncate();
        let mut pos_b = transform_b.translation.truncate();
        let a_to_b = pos_b - pos_a;

        if a_to_b.length() < UNIT_RADIUS + UNIT_SIGHT_RADIUS {
            //unit attack
            *move_type_a = match *move_type_a {
                MoveType::Stationary => MoveType::Attack(self_ref_b),
                MoveType::AttackMove(_) => MoveType::Attack(self_ref_b),
                MoveType::Move(pos) => MoveType::Move(pos),
                MoveType::Attack(id) => MoveType::Attack(id),
            };
            *move_type_b = match *move_type_b {
                MoveType::Stationary => MoveType::Attack(self_ref_a),
                MoveType::AttackMove(_) => MoveType::Attack(self_ref_a),
                MoveType::Move(pos) => MoveType::Move(pos),
                MoveType::Attack(id) => MoveType::Attack(id),
            };
        }
        if a_to_b.length() < UNIT_RADIUS + UNIT_RADIUS {
            //unit collision
            let collide_dist = 2. * UNIT_RADIUS - a_to_b.length();
            let a_to_b_dir = a_to_b.normalize();
            pos_a -= a_to_b_dir * collide_dist / 2.;
            pos_b += a_to_b_dir * collide_dist / 2.;
            transform_a.translation = pos_a.extend(transform_a.translation.z);
            transform_b.translation = pos_b.extend(transform_b.translation.z);
        }
    }
}

fn update_old_pos(mut query: Query<(&Transform, &mut OldPos), With<Unit>>) {
    for (transform, mut old_pos) in &mut query {
        *old_pos = OldPos(trans_to_vec4(transform).truncate().truncate());
    }
}

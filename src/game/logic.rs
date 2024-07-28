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
                units_decide_action,
                units_execute_action,
                store_frame_data,
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
                let vec2 = team_pos + diff * (BASE_RADIUS - LANE_WIDTH);
                SpawnerBundle::new(vec2, team, lane).spawn(&mut root);
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
                    for (transform, team, _lane) in &spawner_query {
                        let vec2 = trans_to_vec4(transform).truncate().truncate();
                        UnitBundle::new(vec2, *team, Discipline::Melee).spawn(&mut root);
                        //TODO: SpatialIndex initialize update
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

fn check_for_attack_targets(
    pos: Vec2,
    team: Team,
    spatial_index: &Res<SpatialIndex>,
    pos_query: &Query<(&Team, &OldPos)>,
) -> Option<Entity> {
    for entity in spatial_index.get_nearby(pos) {
        let (nearby_team, nearby_pos) = pos_query.get(entity).unwrap();
        if team != *nearby_team {
            if (nearby_pos.0 - pos).length() <= UNIT_RADIUS + UNIT_SIGHT_RADIUS {
                return Some(entity);
            }
        }
    }
    return None;
}

fn set_attack_behaviour(
    mut action: Mut<Action>,
    pos_query: &Query<(&Team, &OldPos)>,
    target: Entity,
    pos: Vec2,
) {
    let (_, target_pos) = pos_query.get(target).unwrap();
    //TODO: more complex logic accounting for the fact that timers (sometimes) need to be reset and set to different durations
    //depending on the flip from Attack->Pursue and vice versa
    *action = Action::Attack(
        target,
        if (target_pos.0 - pos).length() <= UNIT_RADIUS + UNIT_ATTACK_RADIUS {
            AttackBehaviour::Attack
        } else {
            AttackBehaviour::Pursue
        },
    );
}

fn units_decide_action(
    mut query: Query<(&Transform, &mut Action, &Team, &mut MidCrossed)>,
    pos_query: Query<(&Team, &OldPos)>,
    spatial_index: Res<SpatialIndex>,
) {
    for (transform, mut action, team, mut mid_crossed) in &mut query {
        let pos = transform.translation.truncate();
        match *action {
            Action::Stop(attack) => {
                if attack == AttackOverride::Attack {
                    if let Some(target) =
                        check_for_attack_targets(pos, *team, &spatial_index, &pos_query)
                    {
                        set_attack_behaviour(action, &pos_query, target, pos);
                    }
                }
            }
            Action::Move(dest, attack) => {
                if attack == AttackOverride::Attack {
                    if let Some(target) =
                        check_for_attack_targets(pos, *team, &spatial_index, &pos_query)
                    {
                        set_attack_behaviour(action, &pos_query, target, pos);
                    }
                } else {
                    if (dest - pos).length() <= UNIT_RADIUS {
                        if mid_crossed.0 {
                            *action = Action::Stop(attack);
                        } else {
                            mid_crossed.0 = true;
                            let new_dest = match *team {
                                Team::Red => BLUE,
                                Team::Blue => RED,
                            };
                            *action = Action::Move(new_dest, attack);
                        }
                    }
                }
            }
            Action::Attack(target, _) => set_attack_behaviour(action, &pos_query, target, pos),
        }
    }
}

fn move_unit(pos: Vec2, dest: Vec2, time: &Res<Time>, mut transform: Mut<Transform>) {
    let wriggle = Vec2::new(
        thread_rng().gen_range(-1.0..=1.0),
        thread_rng().gen_range(-1.0..=1.0),
    ) * UNIT_WRIGGLE;
    let direction = (dest - pos + wriggle).normalize();
    transform.translation += direction.extend(0.) * UNIT_SPEED * time.delta_seconds();
    transform.rotation = Quat::from_rotation_z(direction.to_angle());
    //TODO: collision avoidance logic
}

fn units_execute_action(
    mut query: Query<(&mut Transform, &mut Action, &Lane, &Team, &mut MidCrossed)>,
    pos_query: Query<&OldPos>,
    time: Res<Time>,
) {
    for (mut transform, mut action, lane, team, mut mid_crossed) in &mut query {
        let pos = transform.translation.truncate();
        match *action {
            Action::Stop(attack) => {}
            Action::Move(dest, attack) => {
                move_unit(pos, dest, &time, transform);
            }
            Action::Attack(target, behaviour) => match behaviour {
                AttackBehaviour::Pursue => {
                    let dest = pos_query.get(target).unwrap().0;
                    move_unit(pos, dest, &time, transform);
                }
                AttackBehaviour::Attack => {
                    //TODO: attack logic
                }
            },
        }
    }
}

fn store_frame_data(mut query: Query<(&Transform, &mut OldPos), With<Unit>>) {
    for (transform, mut old_pos) in &mut query {
        *old_pos = OldPos(trans_to_vec4(transform).truncate().truncate());
    }
    //TODO: SpatialIndex movement update (use transform change detection?)
}

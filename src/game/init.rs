use crate::helpers::{consts::*, types::*, utils::*};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::f32::consts::PI;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_assets, (init_map, init_waves)).chain());
    }
}

fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Handles {
        //units
        unit: Mesh2dHandle(meshes.add(Circle::new(UNIT_RADIUS))),
        direction: Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::new(UNIT_RADIUS, 0.),
            Vec2::new(
                -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos(),
                UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.sin(),
            ),
            Vec2::new(
                -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos(),
                -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.sin(),
            ),
        ))),

        //spawner
        spawner: Mesh2dHandle(meshes.add(Circle::new(SPAWNER_RADIUS))),

        //map
        plain: Mesh2dHandle(meshes.add(Rectangle::from_length(MAP_SIZE))),
        river: Mesh2dHandle(meshes.add(Rectangle::new(
            RIVER_WIDTH * MAP_SIZE,
            f32::sqrt(2.) * NON_LANE_WIDTH * MAP_SIZE,
        ))),
        mid: Mesh2dHandle(meshes.add(Rectangle::new(
            LANE_WIDTH * MAP_SIZE,
            f32::sqrt(2.) * NON_LANE_WIDTH * MAP_SIZE,
        ))),
        lane: Mesh2dHandle(meshes.add(Rectangle::new(LANE_WIDTH * MAP_SIZE, MAP_SIZE))),
        base: Mesh2dHandle(meshes.add(CircularSector::from_radians(
            BASE_RADIUS * MAP_SIZE,
            2. * PI / 4.,
        ))),

        //colors
        red: materials.add(Color::hsl(RED_HUE, SATURATION, BRIGHTNESS)),
        green: materials.add(Color::hsl(GREEN_HUE, SATURATION, BRIGHTNESS)),
        blue: materials.add(Color::hsl(BLUE_HUE, SATURATION, BRIGHTNESS)),
        yellow: materials.add(Color::hsl(YELLOW_HUE, SATURATION, BRIGHTNESS)),
        teal: materials.add(Color::hsl(TEAL_HUE, SATURATION, BRIGHTNESS)),
        purple: materials.add(Color::hsl(PURPLE_HUE, SATURATION, BRIGHTNESS)),
    });
}

fn init_map(mut commands: Commands, handles: Res<Handles>) {
    commands.spawn(MaterialMesh2dBundle {
        //plain
        mesh: handles.plain.clone(),
        material: handles.green.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-5.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //river
        mesh: handles.river.clone(),
        material: handles.teal.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-4.))
            .with_rotation(Quat::from_rotation_z(PI / 4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //mid
        mesh: handles.mid.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-3.))
            .with_rotation(Quat::from_rotation_z(7. * PI / 4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //top
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(Vec2::new(0., MID_LANE * MAP_SIZE).extend(-2.))
            .with_rotation(Quat::from_rotation_z(2. * PI / 4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //left
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(Vec2::new(-MID_LANE * MAP_SIZE, 0.).extend(-2.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //bot
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(Vec2::new(0., -MID_LANE * MAP_SIZE).extend(-2.))
            .with_rotation(Quat::from_rotation_z(2. * PI / 4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //right
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(Vec2::new(MID_LANE * MAP_SIZE, 0.).extend(-2.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //red base
        mesh: handles.base.clone(),
        material: handles.red.clone(),
        transform: Transform::from_translation(Vec2::splat(-MAP_SIZE / 2.).extend(-1.))
            .with_rotation(Quat::from_rotation_z(7. * PI / 4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //blue base
        mesh: handles.base.clone(),
        material: handles.blue.clone(),
        transform: Transform::from_translation(Vec2::splat(MAP_SIZE / 2.).extend(-1.))
            .with_rotation(Quat::from_rotation_z(3. * PI / 4.)),
        ..default()
    });
}

fn init_waves(mut commands: Commands, handles: Res<Handles>) {
    //wave manager
    commands.insert_resource(WaveManager::new());

    for (lane_pos, lane) in [
        (Vec2::new(-1., 1.), Lane::Top),
        (Vec2::new(0., 0.), Lane::Mid),
        (Vec2::new(1., -1.), Lane::Bot),
    ] {
        for (team_pos, team) in [
            (Vec2::new(-1., -1.), Team::Red),
            (Vec2::new(1., 1.), Team::Blue),
        ] {
            let diff = (lane_pos - team_pos).normalize();
            let ang = -diff.angle_between(Vec2::X);
            println!(
                "team: {:?}, lane: {:?}, diff: {:?}, ang: {:?}",
                team, lane, diff, ang
            );
            let pos = (team_pos + diff * SPAWNER_POS_RADIUS) * MID_LANE * MAP_SIZE;
            spawn_spawner(
                &mut commands,
                &handles,
                SpawnerBundle::new(pos.extend(0.).extend(ang), team, lane),
            )
        }
    }

    //spawners
    // let red_start = Vec2::splat(-MID_LANE * MAP_SIZE);
    // for dir in [NORTH, NORTH_EAST, EAST] {
    //     let red_pos = red_start + Vec2::from_angle(dir) * SPAWNER_POS_RADIUS * MAP_SIZE;
    //     spawn_spawner(
    //         &mut commands,
    //         &handles,
    //         SpawnerBundle::new(red_pos.x, red_pos.y, dir, Team::Red, SPAWNER_DELAY),
    //     )
    // }
    // let blue_start = Vec2::splat(MID_LANE * MAP_SIZE);
    // for dir in [SOUTH, SOUTH_WEST, WEST] {
    //     let blue_pos = blue_start + Vec2::from_angle(dir) * SPAWNER_POS_RADIUS * MAP_SIZE;
    //     spawn_spawner(
    //         &mut commands,
    //         &handles,
    //         SpawnerBundle::from_xyrt(blue_pos.x, blue_pos.y, dir, Team::Blue, SPAWNER_DELAY),
    //     )
    // }
}

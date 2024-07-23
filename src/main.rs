mod camera;
mod quit;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowMode,
};
use std::f32::consts::PI;

//unit
const UNIT_SPEED: f32 = 300.;
const UNIT_TURN: f32 = PI / 16.;
const UNIT_RADIUS: f32 = 10.; //if set to factor of GCD of SCREEN_WIDTH and SCREEN_HEIGHT, can have a grid with square cells that fits the screen perfectly (currently: 120)
const UNIT_TRIANGLE_ANGLE: f32 = PI / 4.;

//window
const APP_NAME: &str = "Moba MVP";
const SCREEN_WIDTH: f32 = 1920.;
const SCREEN_HEIGHT: f32 = 1080.;

//colors
const SATURATION: f32 = 0.75;
const BRIGHTNESS: f32 = 0.5;
const RED_HUE: f32 = 0.;
const GREEN_HUE: f32 = 120.;
const BLUE_HUE: f32 = 240.;
const TEAL_HUE: f32 = 190.;
const YELLOW_HUE: f32 = 60.;
const PURPLE_HUE: f32 = 275.;

//map
const MAP_SIZE: f32 = SCREEN_HEIGHT;
const LANE_WIDTH: f32 = 0.12;
const INNER_MAP_SIZE: f32 = MAP_SIZE * (1. - 2. * LANE_WIDTH);
const RIVER_WIDTH: f32 = 0.1;
const BASE_RADIUS: f32 = 0.15;

//spawner
const SPAWNER_RADIUS: f32 = 15.;
const SPAWNER_POS_RADIUS: f32 = 0.1;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: APP_NAME.into(),
                    name: Some(APP_NAME.into()),
                    position: WindowPosition::At(IVec2::ZERO),
                    resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
            camera::CameraPlugin,
            quit::QuitPlugin,
        ))
        .add_systems(Startup, (init_assets, (init_map, init_spawners)).chain())
        .add_systems(
            FixedUpdate,
            (update_spawners, update_units_move, update_units_collide).chain(),
        )
        .run()
}

#[derive(Component, PartialEq, Default, Copy, Clone)]
enum Team {
    #[default]
    Red,
    Blue,
}

#[derive(Component, Default)]
struct Unit;
#[derive(Bundle, Default)]
struct UnitBundle {
    spatial: SpatialBundle,
    team: Team,
    unit: Unit, //tag for query filtering
}
impl UnitBundle {
    fn from_xyrt(x: f32, y: f32, r: f32, t: Team) -> Self {
        Self {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.).with_rotation(Quat::from_rotation_z(r)),
                ..default()
            },
            team: t,
            ..default()
        }
    }
}

#[derive(Component, Default)]
struct SpawnTimer(Timer);
#[derive(Component, Default)]
struct Spawner;
#[derive(Bundle, Default)]
struct SpawnerBundle {
    spatial: SpatialBundle,
    team: Team,
    timer: SpawnTimer,
    spawner: Spawner,
}
impl SpawnerBundle {
    fn from_xyrt(x: f32, y: f32, r: f32, t: Team, s: f32) -> Self {
        Self {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(x, y, 0.).with_rotation(Quat::from_rotation_z(r)),
                ..default()
            },
            team: t,
            timer: SpawnTimer(Timer::from_seconds(s, TimerMode::Repeating)),
            ..default()
        }
    }
}

#[derive(Resource)]
struct Handles {
    unit: Mesh2dHandle,
    direction: Mesh2dHandle,
    spawner: Mesh2dHandle,
    plain: Mesh2dHandle,
    river: Mesh2dHandle,
    mid: Mesh2dHandle,
    lane: Mesh2dHandle,
    base: Mesh2dHandle,
    red: Handle<ColorMaterial>,
    green: Handle<ColorMaterial>,
    blue: Handle<ColorMaterial>,
    yellow: Handle<ColorMaterial>,
    teal: Handle<ColorMaterial>,
    purple: Handle<ColorMaterial>,
}

fn spawn_unit(commands: &mut Commands, handles: &Res<Handles>, unit: UnitBundle) {
    let team = unit.team; //avoid borrow checking issue
    commands.spawn(unit).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: handles.unit.clone(), //cloning handles to resources is safe
            material: handles.green.clone(),
            //visibility: Visibility::Hidden, //hide for now
            ..default()
        });
        parent.spawn(MaterialMesh2dBundle {
            mesh: handles.direction.clone(),
            material: if team == Team::Red {
                handles.red.clone()
            } else {
                handles.blue.clone()
            },
            transform: Transform::from_translation(Vec2::ZERO.extend(1.)), //ensure triangles are rendered above circles
            ..default()
        });
    });
}

fn spawn_spawner(commands: &mut Commands, handles: &Res<Handles>, spawner: SpawnerBundle) {
    commands.spawn(spawner).with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: handles.spawner.clone(), //cloning handles to resources is safe
            material: handles.purple.clone(),
            ..default()
        });
    });
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
            f32::sqrt(2.) * INNER_MAP_SIZE,
        ))),
        mid: Mesh2dHandle(meshes.add(Rectangle::new(
            LANE_WIDTH * MAP_SIZE,
            f32::sqrt(2.) * INNER_MAP_SIZE,
        ))),
        lane: Mesh2dHandle(meshes.add(Rectangle::new(LANE_WIDTH * MAP_SIZE, MAP_SIZE))),
        base: Mesh2dHandle(meshes.add(CircularSector::from_radians(
            BASE_RADIUS * MAP_SIZE,
            PI / 2.,
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
            .with_rotation(Quat::from_rotation_z(-PI / 4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //top
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(
            Vec2::new(0., (MAP_SIZE * (1. - LANE_WIDTH)) / 2.).extend(-2.),
        )
        .with_rotation(Quat::from_rotation_z(PI / 2.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //left
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(
            Vec2::new(-((MAP_SIZE * (1. - LANE_WIDTH)) / 2.), 0.).extend(-2.),
        ),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //bot
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(
            Vec2::new(0., -((MAP_SIZE * (1. - LANE_WIDTH)) / 2.)).extend(-2.),
        )
        .with_rotation(Quat::from_rotation_z(PI / 2.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //right
        mesh: handles.lane.clone(),
        material: handles.yellow.clone(),
        transform: Transform::from_translation(
            Vec2::new((MAP_SIZE * (1. - LANE_WIDTH)) / 2., 0.).extend(-2.),
        ),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle {
        //red base
        mesh: handles.base.clone(),
        material: handles.red.clone(),
        transform: Transform::from_translation(Vec2::splat(-MAP_SIZE / 2.).extend(-1.))
            .with_rotation(Quat::from_rotation_z(-PI / 4.)),
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

fn init_spawners(mut commands: Commands, handles: Res<Handles>) {
    let red_spawner = SpawnerBundle::from_xyrt(
        -((MAP_SIZE * (1. - LANE_WIDTH)) / 2.),
        -((MAP_SIZE * (1. - LANE_WIDTH)) / 2.),
        PI / 4.,
        Team::Red,
        1.,
    );
    let blue_spawner = SpawnerBundle::from_xyrt(
        (MAP_SIZE * (1. - LANE_WIDTH)) / 2.,
        (MAP_SIZE * (1. - LANE_WIDTH)) / 2.,
        5. * PI / 4.,
        Team::Blue,
        1.,
    );
    spawn_spawner(&mut commands, &handles, red_spawner);
    spawn_spawner(&mut commands, &handles, blue_spawner);
}

fn update_spawners(
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

fn update_units_move(mut query: Query<&mut Transform, With<Unit>>, time: Res<Time>) {
    for mut transform in &mut query {
        let direction = transform.local_x().as_vec3();
        transform.translation += direction * UNIT_SPEED * time.delta_seconds();
    }
}

fn update_units_collide(mut query: Query<&mut Transform, With<Unit>>) {
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

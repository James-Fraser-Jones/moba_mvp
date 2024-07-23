use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowMode,
};
use std::f32::consts::PI;

use rand::Rng;

//camera
const CAMERA_SPEED: f32 = 500.;
const ZOOM_SPEED: f32 = 0.1;

//unit
const UNIT_SPEED: f32 = 300.;
const UNIT_TURN: f32 = PI / 16.;
const UNIT_RADIUS: f32 = 10.; //if set to factor of GCD of SCREEN_WIDTH and SCREEN_HEIGHT, can have a grid with square cells that fits the screen perfectly (currently: 120)
const UNIT_TRIANGLE_ANGLE: f32 = PI / 4.;

const NUM_UNITS: i32 = 100;

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

//grid
const GRID_SCALE: f32 = 2.; //size of grid cells, relative to unit diameter

//map
const MAP_SIZE: f32 = SCREEN_HEIGHT;
const LANE_WIDTH: f32 = 0.12;
const INNER_MAP_SIZE: f32 = MAP_SIZE * (1. - 2. * LANE_WIDTH);
const RIVER_WIDTH: f32 = 0.1;
const BASE_RADIUS: f32 = 0.15;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_NAME.into(),
                name: Some(APP_NAME.into()),
                position: WindowPosition::At(IVec2::ZERO),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (init_camera, (init_assets, (init_units, init_map)).chain()),
        )
        .add_systems(Update, (quit_game, move_camera))
        .add_systems(FixedUpdate, (move_units, resolve_collisions).chain())
        .run()
}

#[derive(Component, Default)]
struct MainCamera;

#[derive(Component, Default)]
struct Unit;

#[derive(Component, PartialEq, Default, Copy, Clone)]
enum Team {
    #[default]
    Red,
    Blue,
}

#[derive(Bundle, Default)]
struct UnitBundle {
    spatial: SpatialBundle,
    team: Team,
    unit: Unit, //tag for query filtering
}

#[derive(Resource)]
struct Handles {
    unit: Mesh2dHandle,
    direction: Mesh2dHandle,
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
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
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
    });
}

fn init_units(mut commands: Commands, handles: Res<Handles>) {
    let mut rng = rand::thread_rng(); //get ref to random number generator
    for _ in 0..NUM_UNITS {
        let unit = UnitBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(
                    rng.gen_range(-SCREEN_WIDTH..=SCREEN_WIDTH),
                    rng.gen_range(-SCREEN_HEIGHT..=SCREEN_HEIGHT),
                    0.,
                )
                .with_rotation(Quat::from_rotation_z(rng.gen_range((0.)..(2. * PI)))),
                ..default()
            },
            team: if rng.gen() { Team::Blue } else { Team::Red },
            ..default()
        };
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

fn move_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<MainCamera>>,
    mut mouse: EventReader<MouseWheel>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut projection) = query.single_mut(); //okay when entity known to exist and be unique
    let mut direction: Vec2 = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    direction = direction.normalize_or_zero();
    direction *= CAMERA_SPEED * time.delta_seconds();
    transform.translation += direction.extend(0.);
    for scroll_event in mouse.read() {
        if scroll_event.unit == MouseScrollUnit::Line {
            projection.scale -= scroll_event.y * ZOOM_SPEED;
        }
        //pinch zoom unsupported because mobas use mice
    }
}

fn quit_game(mut writer: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

fn move_units(mut query: Query<&mut Transform, With<Unit>>, time: Res<Time>) {
    for mut transform in &mut query {
        //turn units a random amount
        //transform.rotate(Quat::from_rotation_z(rand::thread_rng().gen_range(-UNIT_TURN..=UNIT_TURN)));

        //move units in "forward" direction
        let direction = transform.local_x().as_vec3();
        transform.translation += direction * UNIT_SPEED * time.delta_seconds();

        //wrap units around default camera bounds
        if transform.translation.x > SCREEN_WIDTH / 2. {
            transform.translation.x -= SCREEN_WIDTH;
        } else if transform.translation.x < -SCREEN_WIDTH / 2. {
            transform.translation.x += SCREEN_WIDTH;
        }
        if transform.translation.y > SCREEN_HEIGHT / 2. {
            transform.translation.y -= SCREEN_HEIGHT;
        } else if transform.translation.y < -SCREEN_HEIGHT / 2. {
            transform.translation.y += SCREEN_HEIGHT;
        }
    }
}

fn resolve_collisions(mut query: Query<&mut Transform, With<Unit>>) {
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

fn draw_grid(mut gizmos: Gizmos) {
    let cell_size: f32 = UNIT_RADIUS * 2. * GRID_SCALE;
    gizmos
        .grid_2d(
            Vec2::ZERO,
            0.,
            UVec2::new(
                (SCREEN_WIDTH / cell_size).round() as u32,
                (SCREEN_HEIGHT / cell_size).round() as u32,
            ),
            Vec2::splat(cell_size),
            Color::hsl(GREEN_HUE, SATURATION, BRIGHTNESS),
        )
        .outer_edges();
}

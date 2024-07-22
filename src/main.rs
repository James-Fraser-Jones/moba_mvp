use std::f32::consts::PI;

use bevy::{
    input::mouse::{MouseWheel, MouseScrollUnit}, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::WindowMode
};

use rand::Rng;

//camera
const CAMERA_SPEED: f32 = 500.;
const ZOOM_SPEED: f32 = 0.1;

//unit
const UNIT_SPEED: f32 = 300.;
const UNIT_TURN: f32 = PI/16.;
const UNIT_RADIUS: f32 = 10.; //if set to factor of GCD of SCREEN_WIDTH and SCREEN_HEIGHT, can have a grid with square cells that fits the screen perfectly (currently: 120)
const NUM_UNITS: i32 = 1000;

//screen
const SCREEN_WIDTH: f32 = 1920.;
const SCREEN_HEIGHT: f32 = 1080.;

//cosmetic
const APP_NAME: &str = "Moba MVP";
const ARROW_ANG: f32 = PI/4.;
const SATURATION: f32 = 0.75;
const BRIGHTNESS: f32 = 0.5;
const RED: Color = Color::hsl(0., SATURATION, BRIGHTNESS);
const GREEN: Color = Color::hsl(120., SATURATION, BRIGHTNESS);
const BLUE: Color = Color::hsl(240., SATURATION, BRIGHTNESS);
const TEAL: Color = Color::hsl(190., SATURATION, BRIGHTNESS);
const YELLOW: Color = Color::hsl(60., SATURATION, BRIGHTNESS);

//grid
const GRID_SCALE: f32 = 2.; //size of grid cells, relative to unit diameter

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
        .add_systems(Startup, setup)
        .add_systems(Update, (quit_game, move_camera))
        .add_systems(FixedUpdate, (move_units, resolve_collisions).chain())
        .run()
}

#[derive(Component, Default)]
struct IsMainCamera;

#[derive(Component, Default)]
struct IsUnit;

#[derive(Component, PartialEq, Default, Copy, Clone)]
enum Team {
    #[default] Red,
    Blue,
}

#[derive(Bundle, Default)]
struct UnitBundle {
    spatial: SpatialBundle,
    team: Team,
    is_unit: IsUnit,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //spawn camera
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            ..default()
        }
        , IsMainCamera
    ));

    //initialize mesh and material resources (shared across all units)
    let bound_mesh_handle = Mesh2dHandle(meshes.add(Circle::new(UNIT_RADIUS)));
    let arrow_mesh_handle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::new(UNIT_RADIUS, 0.), 
        Vec2::new(-UNIT_RADIUS*ARROW_ANG.cos(), UNIT_RADIUS*ARROW_ANG.sin()), 
        Vec2::new(-UNIT_RADIUS*ARROW_ANG.cos(), -UNIT_RADIUS*ARROW_ANG.sin())
    )));
    let green_material_handle = materials.add(GREEN);
    let red_material_handle = materials.add(RED);
    let blue_material_handle = materials.add(BLUE);
    let yellow_material_handle = materials.add(YELLOW);
    let teal_material_handle = materials.add(TEAL);

    //spawn entities, including adding 2 mesh bundles as child entities of each unit
    let mut rng = rand::thread_rng(); //get ref to random number generator
    for _ in 0..NUM_UNITS {
        let unit = UnitBundle {
            spatial: SpatialBundle {
                transform: 
                    Transform::from_xyz(
                        rng.gen_range(-SCREEN_WIDTH..=SCREEN_WIDTH), 
                        rng.gen_range(-SCREEN_HEIGHT..=SCREEN_HEIGHT), 
                        0.
                    )
                    .with_rotation(Quat::from_rotation_z(
                        rng.gen_range((0.)..(2.*PI))
                    )),
                ..default()
            },
            team: if rng.gen() {Team::Blue} else {Team::Red},
            ..default()
        };
        let team = unit.team; //avoid borrow checking issue
        commands.spawn(unit).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: bound_mesh_handle.clone(), //cloning handles to resources is safe
                material: green_material_handle.clone(),
                //visibility: Visibility::Hidden, //hide for now
                ..default()
            });
            parent.spawn(MaterialMesh2dBundle {
                mesh: arrow_mesh_handle.clone(),
                material: if team == Team::Red {red_material_handle.clone()} else {blue_material_handle.clone()},
                transform: Transform::from_translation(Vec2::ZERO.extend(1.)), //ensure arrows are rendered above the bounding circle meshes
                ..default()
            });
        });
    }

    //spawn "map"
    let map_size = SCREEN_WIDTH.min(SCREEN_HEIGHT);
    let lane_width = 0.12;
    let inner_map_size = map_size * (1. - 2.*lane_width);
    let river_width = 0.1;
    let base_radius = 0.2;
    commands.spawn(MaterialMesh2dBundle { //outer lanes
        mesh: Mesh2dHandle(meshes.add(Rectangle::from_length(map_size))),
        material: yellow_material_handle.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-5.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle { //jungle
        mesh: Mesh2dHandle(meshes.add(Rectangle::from_length(inner_map_size))),
        material: green_material_handle.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle { //river
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(river_width * map_size, f32::sqrt(2.) * inner_map_size))),
        material: teal_material_handle.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-3.)).with_rotation(Quat::from_rotation_z(PI/4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle { //mid
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(lane_width * map_size, f32::sqrt(2.) * inner_map_size))),
        material: yellow_material_handle.clone(),
        transform: Transform::from_translation(Vec2::ZERO.extend(-2.)).with_rotation(Quat::from_rotation_z(-PI/4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle { //red base
        mesh: Mesh2dHandle(meshes.add(CircularSector::from_radians(base_radius * map_size, PI/2.))),
        material: red_material_handle.clone(),
        transform: Transform::from_translation(Vec2::splat(-map_size/2.).extend(-1.)).with_rotation(Quat::from_rotation_z(-PI/4.)),
        ..default()
    });
    commands.spawn(MaterialMesh2dBundle { //blue base
        mesh: Mesh2dHandle(meshes.add(CircularSector::from_radians(base_radius * map_size, PI/2.))),
        material: blue_material_handle.clone(),
        transform: Transform::from_translation(Vec2::splat(map_size/2.).extend(-1.)).with_rotation(Quat::from_rotation_z(3.*PI/4.)),
        ..default()
    });
}

fn move_camera(
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<IsMainCamera>>,
    mut mouse: EventReader<MouseWheel>, 
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
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

fn quit_game(
    mut writer: EventWriter<AppExit>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

fn move_units(mut query: Query<&mut Transform, With<IsUnit>>, time: Res<Time>) {
    for mut transform in &mut query {
        //turn units a random amount
        //transform.rotate(Quat::from_rotation_z(rand::thread_rng().gen_range(-UNIT_TURN..=UNIT_TURN)));

        //move units in "forward" direction
        let direction = transform.local_x().as_vec3();
        transform.translation += direction * UNIT_SPEED * time.delta_seconds();

        //wrap units around default camera bounds
        if transform.translation.x > SCREEN_WIDTH/2. {
            transform.translation.x -= SCREEN_WIDTH;
        }
        else if transform.translation.x < -SCREEN_WIDTH/2. {
            transform.translation.x += SCREEN_WIDTH;
        }
        if transform.translation.y > SCREEN_HEIGHT/2. {
            transform.translation.y -= SCREEN_HEIGHT;
        }
        else if transform.translation.y < -SCREEN_HEIGHT/2. {
            transform.translation.y += SCREEN_HEIGHT;
        }
    }
}

fn resolve_collisions(mut query: Query<&mut Transform, With<IsUnit>>) {
    let mut transforms = query.iter_combinations_mut(); //combinations don't include pairs of refs to a single entity
    while let Some([mut transform_a, mut transform_b]) = transforms.fetch_next() {
        let mut pos_a = transform_a.translation.truncate();
        let mut pos_b = transform_b.translation.truncate();
        let a_to_b = pos_b - pos_a;
        let collide_dist = 2.*UNIT_RADIUS - a_to_b.length();
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
    let cell_size: f32 = UNIT_RADIUS*2. * GRID_SCALE;
    gizmos.grid_2d(
        Vec2::ZERO,
        0.,
        UVec2::new((SCREEN_WIDTH/cell_size).round() as u32, (SCREEN_HEIGHT/cell_size).round() as u32),
        Vec2::splat(cell_size),
        GREEN
        )
        .outer_edges();
}
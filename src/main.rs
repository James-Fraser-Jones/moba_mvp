use std::f32::consts::PI;

use bevy::{
    prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

const CAMERA_SPEED: f32 = 500.;

const UNIT_SPEED: f32 = 150.;
const ARROW_ANG: f32 = PI/4.;
const RADIUS: f32 = 50.;
const RED_HUE: f32 = 0.;
const GREEN_HUE: f32 = 120.0;
const BLUE_HUE: f32 = 240.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (quit_game, move_camera, move_units))
        .run();
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
    //spawn misc stuff - camera and help text
    commands.spawn(
        TextBundle::from_section("ESC to quit\nW,A,S,D to pan", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.),
                left: Val::Px(12.),
                ..default()
            }),
    );
    commands.spawn((Camera2dBundle::default(), IsMainCamera));

    //initialize mesh and material resources (shared across all units)
    let bound_mesh_handle = Mesh2dHandle(meshes.add(Circle::new(RADIUS)));
    let arrow_mesh_handle = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::new(RADIUS, 0.), 
        Vec2::new(-RADIUS*ARROW_ANG.cos(), RADIUS*ARROW_ANG.sin()), 
        Vec2::new(-RADIUS*ARROW_ANG.cos(), -RADIUS*ARROW_ANG.sin())
    )));
    let bound_material_handle = materials.add(Color::hsl(GREEN_HUE, 0.5, 0.5));
    let red_material_handle = materials.add(Color::hsl(RED_HUE, 0.5, 0.5));
    let blue_material_handle = materials.add(Color::hsl(BLUE_HUE, 0.5, 0.5));

    //initialise units
    let units = [
        UnitBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_rotation_z(0.)),
                ..default()
            },
            ..default()
        },
        UnitBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(200., 0., 0.).with_rotation(Quat::from_rotation_z(PI/2.)),
                ..default()
            },
            team: Team::Blue,
            ..default()
        },
        UnitBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(0., 200., 0.).with_rotation(Quat::from_rotation_z(3.*PI/4.)),
                ..default()
            },
            team: Team::Blue,
            ..default()
        },
        UnitBundle {
            spatial: SpatialBundle {
                transform: Transform::from_xyz(200., 200., 0.).with_rotation(Quat::from_rotation_z(PI)),
                ..default()
            },
            ..default()
        },
    ];

    //spawn entities, including adding 2 mesh bundles as child entities of each unit
    for unit in units.into_iter() {
        let team = unit.team;
        commands.spawn(unit).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: bound_mesh_handle.clone(), //cloning handles to resources is safe
                material: bound_material_handle.clone(),
                ..default()
            });
            parent.spawn(MaterialMesh2dBundle {
                mesh: arrow_mesh_handle.clone(),
                material: if team == Team::Red {red_material_handle.clone()} else {blue_material_handle.clone()},
                transform: Transform::from_xyz(0., 0., 1.), //ensure arrows are rendered above the bounding circle meshes
                ..default()
            });
        });
    }
}

fn move_camera(mut query: Query<&mut Transform, With<IsMainCamera>>, keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    let mut transform = query.single_mut(); //okay when entity known to exist and be unique
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
}

fn quit_game(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}

fn move_units(mut query: Query<&mut Transform, With<IsUnit>>, time: Res<Time>) {
    for mut transform in &mut query {
        let direction = transform.local_x().as_vec3();
        transform.translation += direction * UNIT_SPEED * time.delta_seconds();
    }
}

use std::f32::consts::PI;

use bevy::{
    prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (quit_game, move_camera))
        .run();
}

#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: RADIUS })),
        material: materials.add(Color::hsl(0., 0.7, 0.5)),
        transform: Transform::from_xyz(
            0.,
            0.,
            0.,
        ),
        ..default()
    });

    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 100.))),
    //     material: materials.add(Color::hsl(180., 0.7, 0.5)),
    //     transform: Transform::from_xyz( 
    //         100.,
    //         0.,
    //         0.,
    //     ),
    //     ..default()
    // });

    const ANG: f32 = 2.*PI/16.;
    const RADIUS: f32 = 20.;
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Triangle2d::new(Vec2::new(0., RADIUS), Vec2::new(RADIUS*ANG.sin(), -RADIUS*ANG.cos()), Vec2::new(-RADIUS*ANG.sin(), -RADIUS*ANG.cos())))),
        material: materials.add(Color::hsl(180., 0.7, 0.5)),
        transform: Transform::from_xyz( 
            0.,
            0.,
            1.,
        ),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section("ESC to quit\nW,A,S,D to pan", TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(12.),
                left: Val::Px(12.),
                ..default()
            }),
    );
}

const CAMERA_SPEED: f32 = 500.;

fn move_camera(mut query: Query<&mut Transform, With<MainCamera>>, keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>) {
    for mut transform in &mut query {
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
}

fn quit_game(
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit_events.send(bevy::app::AppExit::Success);
    }
}

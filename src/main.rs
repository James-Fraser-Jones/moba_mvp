use bevy::color::palettes::css;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .run()
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(0.5))),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::ROYAL_BLUE),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.2)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(0.5))),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::DARK_RED),
            unlit: true,
            double_sided: false, //important
            cull_mode: None,     //important
            ..default()
        }),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
        projection: Projection::Perspective(PerspectiveProjection {
            near: 0., //important
            ..default()
        }),
        ..default()
    });
}

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
    //Approx 50% of the time this results in draw errors
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::ROYAL_BLUE),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0., 0., 200.)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::DARK_RED),
            unlit: true,
            double_sided: false,
            cull_mode: None,
            ..default()
        }),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 3000.)),
        projection: Projection::Perspective(PerspectiveProjection {
            near: 0.,
            far: 4000.,
            ..default()
        }),
        ..default()
    });
}

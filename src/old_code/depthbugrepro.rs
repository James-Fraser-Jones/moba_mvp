use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::render::{
    mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology,
};

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
            base_color: Color::Srgba(css::RED),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(custom_plane()),
        material: materials.add(StandardMaterial {
            base_color: Color::Srgba(css::BLUE),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0.5, 0.5, 0.5)),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        ..default()
    });
}

fn custom_plane() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-0.5, 0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
        ],
    )
    // .with_inserted_attribute(
    //     Mesh::ATTRIBUTE_UV_0,
    //     vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
    // )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ],
    )
    .with_inserted_indices(Indices::U32(vec![0, 2, 1, 1, 2, 3]))
}

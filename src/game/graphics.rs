use bevy::{
    math::{Affine2, VectorSpace},
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
        *,
    },
};

use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    let ground_texture: Handle<Image> = server.load_with_settings(
        "textures/kenney_dev_textures/Dark/texture_07.png",
        |settings: &mut texture::ImageLoaderSettings| {
            settings.sampler = texture::ImageSampler::Descriptor(texture::ImageSamplerDescriptor {
                address_mode_u: texture::ImageAddressMode::Repeat,
                address_mode_v: texture::ImageAddressMode::Repeat,
                ..default()
            })
        },
    );
    let ground_material = standard_materials.add(StandardMaterial {
        base_color_texture: Some(ground_texture),
        unlit: true,
        uv_transform: Affine2::from_scale(Vec2::splat(10.)),
        ..default()
    });

    let wall_texture: Handle<Image> = server.load_with_settings(
        "textures/kenney_dev_textures/Orange/texture_07.png",
        |settings: &mut texture::ImageLoaderSettings| {
            settings.sampler = texture::ImageSampler::Descriptor(texture::ImageSamplerDescriptor {
                address_mode_u: texture::ImageAddressMode::Repeat,
                address_mode_v: texture::ImageAddressMode::Repeat,
                ..default()
            })
        },
    );
    let wall_material = standard_materials.add(StandardMaterial {
        base_color_texture: Some(wall_texture),
        unlit: true,
        uv_transform: Affine2::from_scale(Vec2::splat(10.)),
        ..default()
    });

    let color_material = standard_materials.add(StandardMaterial {
        base_color: Color::WHITE,
        unlit: true,
        ..default()
    });

    //ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: ground_material.clone(),
        ..default()
    });

    // //test plane (no rendering issue)
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
    //     material: color_material.clone(),
    //     transform: Transform::from_translation(Vec3::ZERO.with_z(200.)),
    //     ..default()
    // });

    //test manually constructed plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(create_plane()),
        material: color_material.clone(),
        transform: Transform::from_translation(Vec3::ZERO.with_z(200.)),
        ..default()
    });

    // //test gltf scene (has rendering issue)
    // let scene: Handle<Scene> = server.load("models/rift.glb#Scene0");
    // for rot in [0., PI] {
    //     commands.spawn(SceneBundle {
    //         scene: scene.clone(),
    //         transform: Transform::from_translation(Vec3::ZERO.with_z(200.))
    //             .with_rotation(Quat::from_euler(EulerRot::ZYX, rot, 0., PI / 2.)),
    //         ..default()
    //     });
    // }

    // //seperate gltf primitive meshes (has rendering issue)
    // for i in 0..26 {
    //     let mesh: Handle<Mesh> = server.load(format!("models/rift.glb#Mesh{}/Primitive0", i));
    //     for rot in [0., PI] {
    //         commands.spawn(PbrBundle {
    //             mesh: mesh.clone(),
    //             material: color_material.clone(),
    //             transform: Transform::from_translation(Vec3::ZERO.with_z(200.))
    //                 .with_rotation(Quat::from_rotation_z(rot)),
    //             ..default()
    //         });
    //     }
    // }
}

fn update() {}

fn create_plane() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-1000., 1000., 0.],
            [1000., 1000., 0.],
            [-1000., -1000., 0.],
            [1000., -1000., 0.],
        ],
    )
    // .with_inserted_attribute(
    //     Mesh::ATTRIBUTE_UV_0,
    //     vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]],
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
    .with_inserted_indices(Indices::U32(vec![0, 2, 1, 1, 2, 3])) //anti-clockwise cycling of indices required
}

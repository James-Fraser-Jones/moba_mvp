use bevy::{
    math::{Affine2, VectorSpace},
    prelude::*,
    render::{
        mesh::{Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues},
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
        "textures/kenney_dev_textures/Orange/texture_08.png",
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

    // let color_material = standard_materials.add(StandardMaterial {
    //     base_color: Color::WHITE,
    //     unlit: true,
    //     ..default()
    // });

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

    // //test manually constructed plane (no rendering issue unless uvs missing)
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(create_plane()),
    //     material: color_material.clone(),
    //     transform: Transform::from_translation(Vec3::ZERO.with_z(200.)),
    //     ..default()
    // });

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

    // //single mesh from the gltf, for testing
    // commands.spawn(PbrBundle {
    //     mesh: server.load("models/rift.glb#Mesh0/Primitive0"),
    //     material: color_material.clone(),
    //     transform: Transform::from_translation(Vec3::ZERO.with_z(200.)),
    //     ..default()
    // });

    //seperate gltf primitive meshes (has rendering issue)
    for i in 0..26 {
        let mesh: Handle<Mesh> = server.load(format!("models/map.glb#Mesh{}/Primitive0", i));
        for rot in [0., PI] {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: wall_material.clone(),
                transform: Transform::from_rotation(Quat::from_rotation_z(rot)),
                ..default()
            });
        }
    }
}

fn update(
    mut mesh_events: EventReader<AssetEvent<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    server: Res<AssetServer>,
) {
    for mesh_event in mesh_events.read() {
        match *mesh_event {
            AssetEvent::Added { id } => {
                let mesh = meshes.get_mut(id).unwrap();
                if let None = mesh.attribute(Mesh::ATTRIBUTE_UV_0) {
                    let vertices = mesh
                        .attribute(Mesh::ATTRIBUTE_POSITION)
                        .unwrap()
                        .as_float3()
                        .unwrap();
                    let mut uvs = vec![Vec2::ZERO; vertices.len()];
                    for (i, vertex) in vertices.iter().enumerate() {
                        uvs[i] = (Vec2::new(vertex[0], vertex[1]) + 1000.) / 2000.
                        //convert [-1000, 1000] to [0, 1]
                    }
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                }
                if let Some(path) = server.get_path(id) {
                    println!("Loaded external mesh: {}", path);
                }
                //println!("{:?}", mesh);
            }
            _ => {}
        }
    }
}

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
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
    )
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

// this is the custom made plane
// Mesh {
//     primitive_topology: TriangleList,
//     attributes: {
//         MeshVertexAttributeId(0): MeshAttributeData {
//             attribute: MeshVertexAttribute {
//                 name: "Vertex_Position",
//                 id: MeshVertexAttributeId(0),
//                 format: Float32x3
//             },
//             values: Float32x3([[-1000.0, 1000.0, 0.0], [1000.0, 1000.0, 0.0], [-1000.0, -1000.0, 0.0], [1000.0, -1000.0, 0.0]])
//         },
//         MeshVertexAttributeId(1): MeshAttributeData {
//             attribute: MeshVertexAttribute {
//                 name: "Vertex_Normal",
//                 id: MeshVertexAttributeId(1),
//                 format: Float32x3
//             },
//             values: Float32x3([[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]])
//         },
//         MeshVertexAttributeId(2): MeshAttributeData {
//             attribute: MeshVertexAttribute {
//                 name: "Vertex_Uv",
//                 id: MeshVertexAttributeId(2),
//                 format: Float32x2 },
//             values: Float32x2([[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]])
//         }
//     },
//     indices: Some(U32([0, 2, 1, 1, 2, 3])),
//     morph_targets: None,
//     morph_target_names: None,
//     asset_usage: RenderAssetUsages(MAIN_WORLD | RENDER_WORLD)
// }

// //this is an example mesh from the gltf, only difference seems to be that it uses u16 indices
// Mesh {
//     primitive_topology: TriangleList,
//     attributes: {
//         MeshVertexAttributeId(0): MeshAttributeData {
//             attribute: MeshVertexAttribute {
//                 name: "Vertex_Position",
//                 id: MeshVertexAttributeId(0),
//                 format: Float32x3
//             },
//             values: Float32x3([
//                 [-618.67395, -49.375423, 0.0],
//                 [-600.02167, -48.780136, 0.0],
//                 [-598.2358, -86.283165, 0.0],
//                 [-583.35364, -127.3579, 0.0],
//                 [-552.2003, -170.21849, 0.0],
//                 [-508.74442, -205.14195, 0.0],
//                 [-465.48697, -220.61938, 0.0],
//                 [-417.86407, -220.22252, 0.0],
//                 [-388.29822, -211.69008, 0.0],
//                 [-360.5182, -225.18323, 0.0],
//                 [-359.9229, -247.60568, 0.0],
//                 [-451.99384, -330.15204, 0.0],
//                 [-510.13345, -295.03015, 0.0],
//                 [-596.05304, -263.6784, 0.0],
//                 [-587.9175, -225.18323, 0.0],
//                 [-595.45776, -190.65663, 0.0],
//                 [-622.4441, -154.5426, 0.0],
//                 [-627.6032, -114.46001, 0.0]
//             ])
//         },
//         MeshVertexAttributeId(1): MeshAttributeData {
//             attribute: MeshVertexAttribute {
//                 name: "Vertex_Normal",
//                 id: MeshVertexAttributeId(1),
//                 format: Float32x3
//             },
//             values: Float32x3([
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0],
//                 [0.0, 0.0, 1.0]
//             ])
//         }
//     },
//     indices: Some(U16([
//         1, 0, 17,
//         17, 16, 15,
//         14, 13, 12,
//         1, 17, 15,
//         15, 14, 12,
//         12, 11, 10,
//         10, 9, 8,
//         10, 8, 7,
//         15, 12, 10,
//         2, 1, 15,
//         3, 2, 15,
//         10, 7, 6,
//         15, 10, 6,
//         4, 3, 15,
//         15, 6, 5,
//         15, 5, 4
//     ])),
//     morph_targets: None,
//     morph_target_names: None,
//     asset_usage: RenderAssetUsages(MAIN_WORLD | RENDER_WORLD)
// }

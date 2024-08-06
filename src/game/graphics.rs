use bevy::gltf::{GltfMesh, GltfNode};
use bevy::{math::Affine2, prelude::*, render::*};
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, (update, handle_gltf_events));
    }
}

#[derive(Resource)]
struct Gltfs(Vec<Handle<Gltf>>); //handles must be stored somewhere (in a resource or otherwise) or the asset will be dropped and inaccessible via AssetEvents
#[derive(Resource)]
struct DevMaterials {
    ground: Handle<StandardMaterial>,
    wall: Handle<StandardMaterial>,
    area: Handle<StandardMaterial>,
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    //load gltf file(s)
    commands.insert_resource(Gltfs(vec![server.load::<Gltf>("models/map.glb")]));

    //add dev materials
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
    let area_texture: Handle<Image> = server.load_with_settings(
        "textures/kenney_dev_textures/Green/texture_08.png",
        |settings: &mut texture::ImageLoaderSettings| {
            settings.sampler = texture::ImageSampler::Descriptor(texture::ImageSamplerDescriptor {
                address_mode_u: texture::ImageAddressMode::Repeat,
                address_mode_v: texture::ImageAddressMode::Repeat,
                ..default()
            })
        },
    );
    let area_material = standard_materials.add(StandardMaterial {
        base_color_texture: Some(area_texture),
        unlit: true,
        uv_transform: Affine2::from_scale(Vec2::splat(10.)),
        ..default()
    });
    commands.insert_resource(DevMaterials {
        ground: ground_material.clone(),
        wall: wall_material.clone(),
        area: area_material.clone(),
    });

    //add ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: ground_material.clone(),
        ..default()
    });
}

fn update() {}

fn handle_gltf_events(
    mut gltf_events: EventReader<AssetEvent<Gltf>>,
    server: Res<AssetServer>,
    //for working down through the gltf hierarchy to grab bevy Mesh
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    gltf_node_assets: Res<Assets<GltfNode>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    //for spawning a new entity
    mut commands: Commands,
    dev_materials: Res<DevMaterials>,
) {
    for gltf_event in gltf_events.read() {
        match *gltf_event {
            AssetEvent::Added { id } => {
                let path = server.get_path(id).unwrap();
                if path == "models/map.glb".into() {
                    //load map
                    let map = gltf_assets.get(id).unwrap();

                    //load extruded
                    let node_handle = map.named_nodes.get("RedExtruded").unwrap();
                    let node = gltf_node_assets.get(node_handle).unwrap();
                    let gltf_mesh_handle = node.mesh.as_ref().unwrap();
                    let gltf_mesh = gltf_mesh_assets.get(gltf_mesh_handle).unwrap();
                    let gltf_primitive = &gltf_mesh.primitives[0];
                    let mesh_handle = &gltf_primitive.mesh;
                    let mesh = mesh_assets.get_mut(mesh_handle).unwrap();
                    let vertices = mesh
                        .attribute(Mesh::ATTRIBUTE_POSITION)
                        .unwrap()
                        .as_float3()
                        .unwrap();
                    let mut uvs = vec![Vec2::ZERO; vertices.len()];
                    for (i, vertex) in vertices.iter().enumerate() {
                        uvs[i] = (Vec2::new(vertex[0], vertex[1]) + 1000.) / 2000.
                    }
                    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                    for rot in [0., PI] {
                        commands.spawn(PbrBundle {
                            mesh: mesh_handle.clone(),
                            material: dev_materials.wall.clone(),
                            transform: Transform::from_rotation(Quat::from_rotation_z(rot)),
                            ..default()
                        });
                    }

                    // //load triangulated
                    // let node_handle = map.named_nodes.get("RedTriangulated").unwrap();
                    // let node = gltf_node_assets.get(node_handle).unwrap();
                    // for child in node.children.iter() {
                    //     let gltf_mesh_handle = child.mesh.as_ref().unwrap();
                    //     let gltf_mesh = gltf_mesh_assets.get(gltf_mesh_handle).unwrap();
                    //     let gltf_primitive = &gltf_mesh.primitives[0];
                    //     let mesh_handle = &gltf_primitive.mesh;
                    //     let mesh = mesh_assets.get_mut(mesh_handle).unwrap();
                    //     let vertices = mesh
                    //         .attribute(Mesh::ATTRIBUTE_POSITION)
                    //         .unwrap()
                    //         .as_float3()
                    //         .unwrap();
                    //     let mut uvs = vec![Vec2::ZERO; vertices.len()];
                    //     for (i, vertex) in vertices.iter().enumerate() {
                    //         uvs[i] = (Vec2::new(vertex[0], vertex[1]) + 1000.) / 2000.
                    //     }
                    //     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
                    //     for rot in [0., PI] {
                    //         commands.spawn(PbrBundle {
                    //             mesh: mesh_handle.clone(),
                    //             material: dev_materials.area.clone(),
                    //             transform: Transform::from_rotation(Quat::from_rotation_z(rot)),
                    //             ..default()
                    //         });
                    //     }
                    // }
                }
            }
            _ => {}
        }
    }
}

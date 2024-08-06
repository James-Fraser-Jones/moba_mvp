//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::{os::Handles, *};
use bevy::gltf::{GltfMesh, GltfNode};
use bevy::{math::Affine2, prelude::*, render::*};
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.after(os::init));
        app.add_systems(Update, (update, handle_gltf_events));
    }
}

fn init(
    mut commands: Commands,

    image_handles: Res<Handles<Image>>,

    mut material_handles: ResMut<Handles<StandardMaterial>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,

    mut mesh_handles: ResMut<Handles<Mesh>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    //add meshes
    mesh_handles.add(
        "ground_plane",
        Plane3d::new(Vec3::Z, Vec2::splat(1000.)),
        &mut mesh_assets,
    );

    //add materials
    for (material_name, image_path) in [
        ("ground", "textures/kenney_dev_textures/Dark/texture_07.png"),
        ("wall", "textures/kenney_dev_textures/Orange/texture_08.png"),
        ("area", "textures/kenney_dev_textures/Green/texture_08.png"),
    ] {
        material_handles.add(
            &material_name,
            StandardMaterial {
                base_color_texture: Some(image_handles.get(image_path)),
                unlit: true,
                uv_transform: Affine2::from_scale(Vec2::splat(10.)),
                ..default()
            },
            &mut material_assets,
        );
    }

    //spawn ground plane
    commands.spawn(PbrBundle {
        mesh: mesh_handles.get("ground_plane"),
        material: material_handles.get("ground"),
        ..default()
    });

    //spawn map
    commands.spawn
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
    material_handles: Res<Handles<StandardMaterial>>,
) {
    for gltf_event in gltf_events.read() {
        match *gltf_event {
            AssetEvent::Added { id } => {
                let path = server.get_path(id).unwrap();
                if path == "models/map.glb".into() {
                    let map = gltf_assets.get(id).unwrap();
                    let extruded = false;
                    if extruded {
                        let node_handle = map.named_nodes.get("RedExtrudedTriangulated").unwrap();
                        let node = gltf_node_assets.get(node_handle).unwrap();
                        let mesh_handle = get_mesh_handle(node, &gltf_mesh_assets);
                        let mesh = mesh_assets.get_mut(&mesh_handle).unwrap();
                        add_uvs(mesh);
                        spawn_map(
                            &mut commands,
                            mesh_handle.clone(),
                            material_handles.get("wall"),
                        );
                    } else {
                        let node_handle = map.named_nodes.get("RedTriangulated").unwrap();
                        let node = gltf_node_assets.get(node_handle).unwrap();
                        for child in node.children.iter() {
                            let mesh_handle = get_mesh_handle(child, &gltf_mesh_assets);
                            let mesh = mesh_assets.get_mut(&mesh_handle).unwrap();
                            add_uvs(mesh);
                            spawn_map(
                                &mut commands,
                                mesh_handle.clone(),
                                material_handles.get("area"),
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn get_mesh_handle(node: &GltfNode, gltf_mesh_assets: &Assets<GltfMesh>) -> Handle<Mesh> {
    let gltf_mesh_handle = node.mesh.as_ref().unwrap();
    let gltf_mesh = gltf_mesh_assets.get(gltf_mesh_handle).unwrap();
    let gltf_primitive = &gltf_mesh.primitives[0];
    gltf_primitive.mesh.clone()
}

fn add_uvs(mesh_handle: &mut Handle<Mesh>) {
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
}

fn spawn_map(commands: &mut Commands, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) {
    commands.spawn(PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_rotation(Quat::from_rotation_z(0.)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh,
        material,
        transform: Transform::from_rotation(Quat::from_rotation_z(PI)),
        ..default()
    });
}

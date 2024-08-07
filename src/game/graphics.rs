//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::{os::Handles, *};
use bevy::gltf::GltfMesh;
use bevy::{math::Affine2, prelude::*, render::*};
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_resources, init).chain().after(os::init));
        app.add_systems(Update, (update, add_map));
    }
}

// #[derive(Resource)]
// struct GraphicsSettings {
// }
// impl Default for GraphicsSettings {
//     fn default() -> Self {
//         Self {}
//     }
// }

pub const BLENDER_WALL_HEIGHT: f32 = 20.;
const WALL_HEIGHT: f32 = 30.;

#[derive(Component, Default)]
pub struct Wall;

fn init_resources(mut commands: Commands) {}

fn init(
    mut commands: Commands,
    image_handles: Res<Handles<Image>>,

    mut material_handles: ResMut<Handles<StandardMaterial>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,

    mut mesh_handles: ResMut<Handles<Mesh>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    //add meshes
    mesh_handles.add_asset(
        &mut mesh_assets,
        "ground_plane",
        Plane3d::new(Vec3::Z, Vec2::splat(1000.)),
    );

    //add materials
    for (material_name, image_name) in [
        ("ground", "dev_dark"),
        ("wall", "dev_orange"),
        ("area", "dev_green"),
    ] {
        material_handles.add_asset(
            &mut material_assets,
            &material_name,
            StandardMaterial {
                base_color_texture: Some(image_handles.get_handle(image_name).clone()),
                unlit: true,
                uv_transform: Affine2::from_scale(Vec2::splat(10.)),
                ..default()
            },
        );
    }

    //spawn ground plane
    commands.spawn(PbrBundle {
        mesh: mesh_handles.get_handle("ground_plane").clone(),
        material: material_handles.get_handle("ground").clone(),
        ..default()
    });
}

fn update() {}

fn add_map(
    mut commands: Commands,
    mut gltf_events: EventReader<AssetEvent<Gltf>>,
    server: Res<AssetServer>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    material_handles: Res<Handles<StandardMaterial>>,
) {
    for gltf_event in gltf_events.read() {
        match *gltf_event {
            AssetEvent::Added { id } => {
                let path = server.get_path(id).unwrap();
                if path == "models/map.glb".into() {
                    let map = gltf_assets.get(id).unwrap();
                    for gltf_mesh_handle in &map.meshes {
                        let gltf_primitive =
                            &gltf_mesh_assets.get(gltf_mesh_handle).unwrap().primitives[0];
                        if gltf_primitive.name.contains("Extruded") {
                            //add uvs
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
                            //spawn mesh bundles
                            for rot in [0., PI] {
                                commands.spawn((
                                    PbrBundle {
                                        mesh: mesh_handle.clone(),
                                        material: material_handles.get_handle("wall").clone(),
                                        transform: Transform::from_rotation(Quat::from_rotation_z(
                                            rot,
                                        ))
                                        .with_scale(Vec3::new(
                                            1.,
                                            1.,
                                            (1. / BLENDER_WALL_HEIGHT) * WALL_HEIGHT,
                                        )),
                                        ..default()
                                    },
                                    Wall,
                                ));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

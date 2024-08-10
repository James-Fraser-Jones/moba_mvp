//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::*;
use bevy::{color::palettes::css, pbr::wireframe::Wireframe, prelude::*, render::*};
use logic::Team;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColoredMaterialMap>();
        app.init_resource::<AllowedMeshMap>();
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

pub const WALL_HEIGHT: f32 = 30.;
pub const BLENDER_WALL_HEIGHT: f32 = 50.;

#[derive(Component, Default)]
pub struct Map;

//HandleMap keys
#[derive(PartialEq, Eq, Hash)]
enum OrderedMeshType {
    Sphere,
    Cylinder,
    Capsule,
}
#[derive(PartialEq, Eq, Hash)]
struct OrderedMesh {
    mesh_type: OrderedMeshType,
    half_width: OrderedFloat<f32>,
    half_height: OrderedFloat<f32>,
    half_depth: OrderedFloat<f32>,
}
impl From<AllowedMesh> for OrderedMesh {
    fn from(value: AllowedMesh) -> Self {
        match value {
            AllowedMesh::Sphere(Sphere { radius }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                half_width: OrderedFloat(radius),
                half_height: OrderedFloat(radius),
                half_depth: OrderedFloat(radius),
            },
            AllowedMesh::Cylinder(Cylinder {
                radius,
                half_height,
            }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                half_width: OrderedFloat(radius),
                half_height: OrderedFloat(half_height),
                half_depth: OrderedFloat(radius),
            },
            AllowedMesh::Capsule(Capsule3d {
                radius,
                half_length,
            }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                half_width: OrderedFloat(radius),
                half_height: OrderedFloat(half_length),
                half_depth: OrderedFloat(radius),
            },
            AllowedMesh::Cuboid(Cuboid { half_size }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                half_width: OrderedFloat(half_size.x),
                half_height: OrderedFloat(half_size.z),
                half_depth: OrderedFloat(half_size.y),
            },
        }
    }
}
#[derive(PartialEq, Eq, Hash)]
struct OrderedColor {
    red: OrderedFloat<f32>,
    green: OrderedFloat<f32>,
    blue: OrderedFloat<f32>,
    alpha: OrderedFloat<f32>,
}
impl From<Color> for OrderedColor {
    fn from(value: Color) -> Self {
        let linear: LinearRgba = value.into();
        Self {
            red: OrderedFloat(linear.red),
            green: OrderedFloat(linear.green),
            blue: OrderedFloat(linear.blue),
            alpha: OrderedFloat(linear.alpha),
        }
    }
}

//convertable into *both* keys and assets
#[derive(Copy, Clone)]
pub enum AllowedMesh {
    Sphere(Sphere),
    Cylinder(Cylinder),
    Capsule(Capsule3d),
    Cuboid(Cuboid),
}
impl Into<Mesh> for AllowedMesh {
    fn into(self) -> Mesh {
        match self {
            AllowedMesh::Sphere(sphere) => sphere.into(),
            AllowedMesh::Cylinder(cylinder) => cylinder.into(),
            AllowedMesh::Capsule(capsule) => capsule.into(),
            AllowedMesh::Cuboid(cuboid) => cuboid.into(),
        }
    }
}
fn color_into_unlit(color: Color) -> StandardMaterial {
    StandardMaterial {
        unlit: true,
        ..StandardMaterial::from_color(color)
    }
}

//HandleMap wrappers for Meshes and Materials
#[derive(Resource, Default)]
struct ColoredMaterialMap(os::HandleMap<OrderedColor, StandardMaterial>);
impl ColoredMaterialMap {
    fn clone_material_handle(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        color: Color,
    ) -> Handle<StandardMaterial> {
        if let Some(handle) = self.0 .0.get(&color.into()) {
            handle.clone()
        } else {
            let handle = self
                .0
                .insert_asset(materials, color.into(), color_into_unlit(color));
            handle.clone()
        }
    }
}
#[derive(Resource, Default)]
struct AllowedMeshMap(os::HandleMap<OrderedMesh, Mesh>);
impl AllowedMeshMap {
    fn clone_mesh_handle(
        &mut self,
        meshes: &mut Assets<Mesh>,
        allowed_mesh: AllowedMesh,
    ) -> Handle<Mesh> {
        if let Some(handle) = self.0 .0.get(&allowed_mesh.into()) {
            handle.clone()
        } else {
            let handle = self
                .0
                .insert_asset(meshes, allowed_mesh.into(), allowed_mesh);
            handle.clone()
        }
    }
}

//single display component to be called from logic
#[derive(Component)]
pub struct Display {
    pub allowed_mesh: AllowedMesh,
    pub color: Color,
    pub wireframe: bool,
}

fn init(mut commands: Commands, server: Res<AssetServer>, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::Srgba(css::FOREST_GREEN);
    commands.spawn((
        SceneBundle {
            scene: server.load("models/map.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(1., 1., WALL_HEIGHT / BLENDER_WALL_HEIGHT)),
            ..default()
        },
        Map,
    ));
}

fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Display), Added<Display>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_map: ResMut<ColoredMaterialMap>,
    mut mesh_map: ResMut<AllowedMeshMap>,
) {
    for (entity, display) in &mut query {
        let child = PbrBundle {
            mesh: mesh_map.clone_mesh_handle(&mut meshes, display.allowed_mesh),
            material: material_map.clone_material_handle(&mut materials, display.color),
            transform: match display.allowed_mesh {
                AllowedMesh::Sphere(_) => Transform::default(),
                AllowedMesh::Cylinder(_) => {
                    Transform::from_rotation(Quat::from_rotation_x(PI / 2.))
                }
                AllowedMesh::Capsule(_) => Transform::default(),
                AllowedMesh::Cuboid(_) => Transform::default(),
            },
            ..default()
        };
        if display.wireframe {
            let child = commands.spawn((child, Wireframe)).id();
            commands.entity(entity).add_child(child);
        } else {
            let child = commands.spawn(child).id();
            commands.entity(entity).add_child(child);
        }
    }
}

pub fn team_color(team: logic::Team) -> Color {
    Color::Srgba(match team {
        Team::Red => css::DARK_RED,
        Team::Blue => css::DARK_BLUE,
    })
}

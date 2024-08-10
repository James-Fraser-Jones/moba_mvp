//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::*;
use bevy::{color::palettes::css, prelude::*, render::*};
use ordered_float::OrderedFloat;

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
    radius: OrderedFloat<f32>,
    height: OrderedFloat<f32>,
}
impl From<AllowedMesh> for OrderedMesh {
    fn from(value: AllowedMesh) -> Self {
        match value {
            AllowedMesh::Sphere(Sphere { radius }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                radius: OrderedFloat(radius),
                height: OrderedFloat(radius * 2.),
            },
            AllowedMesh::Cylinder(Cylinder {
                radius,
                half_height,
            }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                radius: OrderedFloat(radius),
                height: OrderedFloat(half_height * 2.),
            },
            AllowedMesh::Capsule(Capsule3d {
                radius,
                half_length,
            }) => OrderedMesh {
                mesh_type: OrderedMeshType::Sphere,
                radius: OrderedFloat(radius),
                height: OrderedFloat(half_length * 2.),
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
}
impl Into<Mesh> for AllowedMesh {
    fn into(self) -> Mesh {
        match self {
            AllowedMesh::Sphere(sphere) => sphere.into(),
            AllowedMesh::Cylinder(cylinder) => cylinder.into(),
            AllowedMesh::Capsule(capsule) => capsule.into(),
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
}

#[derive(Bundle)]
pub struct DisplayBundle {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
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
        commands.entity(entity).insert(DisplayBundle {
            mesh: mesh_map.clone_mesh_handle(&mut meshes, display.allowed_mesh),
            material: material_map.clone_material_handle(&mut materials, display.color),
        });
    }
}

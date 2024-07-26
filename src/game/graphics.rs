use crate::game::consts::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_assets);
        app.add_systems(Update, (handle_mesh_requests, handle_material_requests));
    }
}

#[derive(Resource)]
struct Meshes(HashMap<&'static str, Handle<Mesh>>);

#[derive(Resource)]
struct Materials(HashMap<&'static str, Handle<ColorMaterial>>);

#[derive(Component, Default)]
struct RequestMesh(&'static str);

#[derive(Component, Default)]
struct RequestMaterial(&'static str);

#[derive(Bundle, Default)]
pub struct MeshBundle {
    material_mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
    request_mesh: RequestMesh,
    request_material: RequestMaterial,
}
impl MeshBundle {
    pub fn new(mesh: &'static str, material: &'static str, transform: Transform) -> Self {
        Self {
            request_mesh: RequestMesh(mesh),
            request_material: RequestMaterial(material),
            material_mesh_2d_bundle: MaterialMesh2dBundle {
                transform,
                ..default()
            },
            ..default()
        }
    }
}

fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Meshes(HashMap::from([
        ("plain", meshes.add(Rectangle::from_length(2000.))),
        (
            "river",
            meshes.add(Rectangle::new(
                RIVER_WIDTH,
                f32::sqrt(2.) * NON_LANE_RADIUS * 2.,
            )),
        ),
        (
            "mid",
            meshes.add(Rectangle::new(
                LANE_WIDTH,
                f32::sqrt(2.) * NON_LANE_RADIUS * 2.,
            )),
        ),
        ("lane", meshes.add(Rectangle::new(LANE_WIDTH, 2000.))),
        (
            "base",
            meshes.add(CircularSector::from_radians(BASE_RADIUS, 2. * PI / 4.)),
        ),
        ("spawner", meshes.add(Circle::new(SPAWNER_RADIUS))),
        ("unit", meshes.add(Circle::new(UNIT_RADIUS))),
        (
            "direction",
            meshes.add(Triangle2d::new(
                Vec2::new(UNIT_RADIUS, 0.),
                Vec2::new(
                    -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos(),
                    UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.sin(),
                ),
                Vec2::new(
                    -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos(),
                    -UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.sin(),
                ),
            )),
        ),
    ])));

    commands.insert_resource(Materials(HashMap::from([
        (
            "red",
            materials.add(Color::hsl(RED_HUE, SATURATION, BRIGHTNESS)),
        ),
        (
            "green",
            materials.add(Color::hsl(GREEN_HUE, SATURATION, BRIGHTNESS)),
        ),
        (
            "blue",
            materials.add(Color::hsl(BLUE_HUE, SATURATION, BRIGHTNESS)),
        ),
        (
            "yellow",
            materials.add(Color::hsl(YELLOW_HUE, SATURATION, BRIGHTNESS)),
        ),
        (
            "teal",
            materials.add(Color::hsl(TEAL_HUE, SATURATION, BRIGHTNESS)),
        ),
        (
            "purple",
            materials.add(Color::hsl(PURPLE_HUE, SATURATION, BRIGHTNESS)),
        ),
    ])));
}

fn handle_mesh_requests(
    mut commands: Commands,
    meshes: Res<Meshes>,
    mut query: Query<(Entity, &mut Mesh2dHandle, &mut RequestMesh)>,
) {
    for (entity, mut mesh_2d_handle, request_mesh) in &mut query {
        if let Some(handle) = meshes.0.get(request_mesh.0) {
            (*mesh_2d_handle).0 = handle.clone();
        }
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<RequestMesh>();
        }
    }
}

fn handle_material_requests(
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(Entity, &mut Handle<ColorMaterial>, &mut RequestMaterial)>,
) {
    for (entity, mut handle_color_material, request_material) in &mut query {
        if let Some(handle) = materials.0.get(request_material.0) {
            *handle_color_material = handle.clone();
        }
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<RequestMaterial>();
        }
    }
}

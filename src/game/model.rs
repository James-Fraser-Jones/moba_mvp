use crate::game::*;
use bevy::color::palettes::css;
use bevy::{pbr::wireframe::Wireframe, prelude::*};
use ordered_float::OrderedFloat;
use std::f32::consts::PI;

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaterialMap>();
        app.init_resource::<MeshMap>();
        app.add_systems(Startup, init);
        app.add_systems(Update, add_models.in_set(UpdateGraphics));
    }
}

//colors
const RED_TEAM_COLOR: Color = Color::Srgba(css::TOMATO);
const BLUE_TEAM_COLOR: Color = Color::Srgba(css::DEEP_SKY_BLUE);
const NO_TEAM_COLOR: Color = Color::Srgba(css::SEA_GREEN);
pub fn team_color(team: Option<Team>) -> Color {
    match team {
        Some(team) => match team {
            Team::Red => RED_TEAM_COLOR,
            Team::Blue => BLUE_TEAM_COLOR,
        },
        None => NO_TEAM_COLOR,
    }
}

//textures
#[derive(Resource)]
struct DevTexture(Handle<Image>);

//materials
#[derive(Resource, Default)]
struct MaterialMap(os::HandleMap<HashableColor, StandardMaterial>);
impl MaterialMap {
    fn clone_material_handle(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
        color: Color,
        texture: Option<&Handle<Image>>,
    ) -> Handle<StandardMaterial> {
        if let Some(handle) = self.0 .0.get(&color.into()) {
            handle.clone()
        } else {
            let handle = self.0.insert_asset(
                materials,
                color.into(),
                color_into_unlit(color, texture.map(|t| t.clone())),
            );
            handle.clone()
        }
    }
}
#[derive(PartialEq, Eq, Hash)]
struct HashableColor {
    red: OrderedFloat<f32>,
    green: OrderedFloat<f32>,
    blue: OrderedFloat<f32>,
    alpha: OrderedFloat<f32>,
}
impl From<Color> for HashableColor {
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
fn color_into_unlit(color: Color, texture: Option<Handle<Image>>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: texture,
        unlit: true,
        ..StandardMaterial::from_color(color)
    }
}

//meshes
#[derive(Resource, Default)]
struct MeshMap(os::HandleMap<HashableMesh, Mesh>);
impl MeshMap {
    fn clone_mesh_handle(
        &mut self,
        meshes: &mut Assets<Mesh>,
        allowed_mesh: HashableMesh,
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct HashableMesh {
    mesh_type: HashableMeshType,
    radius: OrderedFloat<f32>,
    half_height: OrderedFloat<f32>,
}
impl HashableMesh {
    fn new(mesh_type: HashableMeshType, radius: f32, half_height: f32) -> Self {
        Self {
            mesh_type,
            radius: OrderedFloat(radius),
            half_height: OrderedFloat(half_height),
        }
    }
}
impl Into<Mesh> for HashableMesh {
    fn into(self) -> Mesh {
        match self.mesh_type {
            HashableMeshType::Capsule => {
                Capsule3d::new(self.radius.0, (self.half_height.0 - self.radius.0) * 2.).into()
            }
            HashableMeshType::Cylinder => {
                Cylinder::new(self.radius.0, self.half_height.0 * 2.).into()
            }
            HashableMeshType::Cuboid => Cuboid::new(
                self.radius.0 * 2.,
                self.radius.0 * 2.,
                self.half_height.0 * 2.,
            )
            .into(),
        }
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
enum HashableMeshType {
    Capsule,
    Cylinder,
    #[default]
    Cuboid,
}

//models
#[derive(Component, Copy, Clone)]
pub struct DisplayModel {
    mesh_type: HashableMeshType,
    half_height_ratio: f32,
    raised: bool,
    wireframe: bool,
}
impl Default for DisplayModel {
    fn default() -> Self {
        Self {
            mesh_type: HashableMeshType::default(),
            half_height_ratio: 1.0,
            raised: true,
            wireframe: false,
        }
    }
}
impl DisplayModel {
    pub fn cuboid() -> Self {
        Self { ..default() }
    }
    pub fn cube() -> Self {
        Self::cuboid()
    }
    pub fn capsule() -> Self {
        Self {
            mesh_type: HashableMeshType::Capsule,
            ..default()
        }
    }
    pub fn sphere() -> Self {
        Self::capsule()
    }
    pub fn hemisphere() -> Self {
        Self::sphere().unraised()
    }
    pub fn cylinder() -> Self {
        Self {
            mesh_type: HashableMeshType::Cylinder,
            ..default()
        }
    }
    pub fn unraised(self) -> Self {
        Self {
            raised: false,
            ..self
        }
    }
    pub fn wireframed(self) -> Self {
        Self {
            wireframe: true,
            ..self
        }
    }
    pub fn with_height_ratio(self, half_height_ratio: f32) -> Self {
        Self {
            half_height_ratio,
            ..self
        }
    }
    pub fn get_height(&self, radius: f32) -> f32 {
        self.half_height_ratio * radius * if self.raised { 2. } else { 1. }
    }
}

fn init(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(DevTexture(
        server.load("textures/untracked/kenney_dev_textures/Light/texture_07.png"),
    ));
}

fn add_models(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DisplayModel, &Radius, Option<&Team>), Added<DisplayModel>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_map: ResMut<MaterialMap>,
    mut mesh_map: ResMut<MeshMap>,
    dev_texture: Res<DevTexture>,
) {
    for (entity, display, radius, team) in &mut query {
        let half_height = display.half_height_ratio * radius.0;
        let mesh = HashableMesh::new(display.mesh_type, radius.0, half_height);
        let material_color = team_color(team.copied());
        let material_texture = &dev_texture.0;

        let model_bundle = PbrBundle {
            mesh: mesh_map.clone_mesh_handle(&mut meshes, mesh),
            material: material_map.clone_material_handle(
                &mut materials,
                material_color,
                Some(material_texture),
            ),
            transform: Transform::from_translation(Vec3::ZERO.with_z(if display.raised {
                half_height
            } else {
                0.
            }))
            .with_rotation(Quat::from_rotation_x(
                if display.mesh_type == HashableMeshType::Cuboid {
                    0.
                } else {
                    PI / 2.
                },
            )),
            ..default()
        };
        let mut model = commands.spawn(model_bundle);
        if display.wireframe {
            model.insert(Wireframe);
        }
        let model_id = model.id();
        commands.entity(entity).add_child(model_id);
    }
}

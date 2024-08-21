use crate::*;
use bevy::{color::palettes::css, prelude::*};
use ordered_float::OrderedFloat;
use std::{collections::HashMap, f32::consts::PI};

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaterialMap>();
        app.init_resource::<MeshMap>();
        app.add_systems(Startup, init.in_set(ModelSet));
        app.add_systems(Update, (add_models, interpolate_models).in_set(ModelSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelSet;

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

//assets
#[derive(Resource)]
pub struct HandleMap<K: Eq + std::hash::Hash, A: Asset>(pub HashMap<K, Handle<A>>);
impl<K: Eq + std::hash::Hash, A: Asset> HandleMap<K, A> {
    pub fn insert_asset(
        &mut self,
        assets: &mut Assets<A>,
        key: K,
        value: impl Into<A>,
    ) -> &Handle<A> {
        let handle = assets.add(value);
        self.0.entry(key).or_insert(handle)
    }
    pub fn insert_asset_path(&mut self, server: &AssetServer, key: K, path: &str) -> &Handle<A> {
        let handle = server.load(path.to_string());
        self.0.entry(key).or_insert(handle)
    }
    pub fn get_asset<'a>(&self, assets: &'a mut Assets<A>, key: &K) -> Option<&'a A> {
        let handle = self.0.get(key)?;
        assets.get(handle)
    }
    pub fn get_asset_mut<'a>(&self, assets: &'a mut Assets<A>, key: &K) -> Option<&'a mut A> {
        let handle = self.0.get(key)?;
        assets.get_mut(handle)
    }
}
impl<K: Eq + std::hash::Hash, A: Asset> Default for HandleMap<K, A> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

//materials
#[derive(Resource, Default)]
struct MaterialMap(HandleMap<HashableColor, StandardMaterial>);
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
struct MeshMap(HandleMap<HashableMesh, Mesh>);
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum HashableMeshType {
    Capsule,
    Cylinder,
    Cuboid,
}

//models
#[derive(Component)]
pub struct Model {
    pub target: Entity,
    old_transform: Transform,
    world_height: f32,
}
#[derive(Bundle)]
struct ModelBundle {
    model: Model,
    spatial: SpatialBundle,
}
impl ModelBundle {
    fn new(target: Entity, transform: Transform, world_height: f32) -> Self {
        Self {
            model: Model {
                target,
                old_transform: transform,
                world_height,
            },
            spatial: SpatialBundle::from_transform(transform),
        }
    }
}

//render data
struct RenderData {
    mesh_type: HashableMeshType,
    half_height_ratio: f32,
    raised: bool,
    alpha: f32,
}
impl RenderData {
    fn new(mesh_type: HashableMeshType, half_height_ratio: f32, raised: bool, alpha: f32) -> Self {
        Self {
            mesh_type,
            half_height_ratio,
            raised,
            alpha,
        }
    }
}
impl From<OutputHandle> for RenderData {
    fn from(output: OutputHandle) -> Self {
        match output {
            OutputHandle::Core => Self::new(HashableMeshType::Capsule, 1.0, false, 1.0),
            OutputHandle::Spawner => Self::new(HashableMeshType::Capsule, 1.0, false, 0.8),
            OutputHandle::Tower => Self::new(HashableMeshType::Cylinder, 1.5, true, 1.0),
            OutputHandle::Advocate => Self::new(HashableMeshType::Capsule, 1.75, true, 1.0),
            OutputHandle::Minion => Self::new(HashableMeshType::Cuboid, 1.0, true, 1.0),
            OutputHandle::Monster => Self::new(HashableMeshType::Capsule, 1.75, true, 1.0),
            OutputHandle::Demon => Self::new(HashableMeshType::Capsule, 1.9, true, 1.0),
        }
    }
}

//TODO: use a handlemap!
fn init(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(DevTexture(
        server.load("textures/untracked/kenney_dev_textures/Light/texture_07.png"),
    ));
}

fn add_models(
    mut commands: Commands,
    object_query: Query<
        (Entity, &OutputHandle, &Radius, Option<&Team>, &Transform),
        Added<OutputHandle>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_map: ResMut<MaterialMap>,
    mut mesh_map: ResMut<MeshMap>,
    dev_texture: Res<DevTexture>,
) {
    for (target, output, radius, team, transform) in &object_query {
        let render_data = RenderData::from(*output);
        let half_height = render_data.half_height_ratio * radius.0;
        //get model bundle
        let world_height = half_height * if render_data.raised { 2.0 } else { 1.0 };
        let model_bundle = ModelBundle::new(target, *transform, world_height);
        //get render bundle
        let mesh = HashableMesh::new(render_data.mesh_type, radius.0, half_height);
        let material_color = team_color(team.copied()).with_alpha(render_data.alpha);
        let render_bundle = PbrBundle {
            mesh: mesh_map.clone_mesh_handle(&mut meshes, mesh),
            material: material_map.clone_material_handle(
                &mut materials,
                material_color,
                Some(&dev_texture.0),
            ),
            transform: Transform::from_translation(Vec3::ZERO.with_z(if render_data.raised {
                half_height
            } else {
                0.
            }))
            .with_rotation(Quat::from_rotation_x(
                if render_data.mesh_type == HashableMeshType::Cuboid {
                    0.
                } else {
                    PI / 2.
                },
            )),
            ..default()
        };
        //spawn model
        commands.spawn(model_bundle).with_children(|builder| {
            builder.spawn(render_bundle);
        });
    }
}

fn interpolate_models(
    mut model_query: Query<(&mut Transform, &Model)>,
    object_query: Query<&Transform, Without<Model>>,
) {
    for (mut model_transform, model) in &mut model_query {
        let object_transform = object_query.get(model.target).unwrap();
        *model_transform = *object_transform;
    }
}

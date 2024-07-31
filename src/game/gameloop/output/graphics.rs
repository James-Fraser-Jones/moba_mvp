use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, (handle_mesh_requests, handle_material_requests));
    }
}

#[derive(Resource)]
pub struct GraphicsMaterialSettings {
    saturation: f32,
    luminance: f32,
    unlit: bool,
    hues: HashMap<&'static str, f32>,
}
impl Default for GraphicsMaterialSettings {
    fn default() -> Self {
        Self {
            saturation: 0.75,
            luminance: 0.5,
            unlit: true,
            hues: HashMap::from([
                ("red", 0.),
                ("green", 120.),
                ("blue", 240.),
                ("teal", 190.),
                ("yellow", 60.),
                ("purple", 275.),
            ]),
        }
    }
}

#[derive(Resource)]
pub struct GraphicsMapSettings {
    spawner_radius: f32,
    river_width: f32,
    base_radius: f32,
    unit_angle: f32,
}
impl Default for GraphicsMapSettings {
    fn default() -> Self {
        Self {
            spawner_radius: 27.8,
            river_width: 200.,
            base_radius: 360.,
            unit_angle: PI / 8.,
        }
    }
}

#[derive(Resource)]
struct Meshes(HashMap<&'static str, Handle<Mesh>>);

#[derive(Resource)]
struct Materials(HashMap<&'static str, Handle<StandardMaterial>>);

#[derive(Component, Default)]
struct RequestMesh(&'static str);

#[derive(Component, Default)]
struct RequestMaterial(&'static str);

#[derive(Bundle, Default)]
pub struct MeshBundle {
    pbr_bundle: PbrBundle,
    request_mesh: RequestMesh,
    request_material: RequestMaterial,
}
impl MeshBundle {
    pub fn new(mesh: &'static str, material: &'static str, transform: Transform) -> Self {
        Self {
            request_mesh: RequestMesh(mesh),
            request_material: RequestMaterial(material),
            pbr_bundle: PbrBundle {
                transform,
                ..default()
            },
            ..default()
        }
    }
}

fn add_lights(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 2000.,
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 1.0, -PI / 4.)),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.init_resource::<GraphicsMaterialSettings>();
    commands.init_resource::<GraphicsMapSettings>();

    commands.insert_resource(Meshes(HashMap::from([
        (
            "plain",
            meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        ),
        (
            "river",
            meshes.add(Cuboid::new(
                RIVER_WIDTH,
                f32::sqrt(2.) * NON_LANE_RADIUS * 2.,
                5.0,
            )),
        ),
        (
            "mid",
            meshes.add(Cuboid::new(
                LANE_WIDTH,
                f32::sqrt(2.) * NON_LANE_RADIUS * 2.,
                10.0,
            )),
        ),
        ("lane", meshes.add(Cuboid::new(LANE_WIDTH, 2000., 10.0))),
        (
            "base",
            meshes.add(Extrusion::new(
                CircularSector::from_radians(BASE_RADIUS, 2. * PI / 4.),
                12.,
            )),
        ),
        ("spawner", meshes.add(Sphere::new(SPAWNER_RADIUS))),
        ("unit", meshes.add(Sphere::new(UNIT_RADIUS))),
        (
            "direction",
            meshes.add(Cone {
                radius: 2. * UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos() * UNIT_TRIANGLE_ANGLE.sin(),
                height: 2. * UNIT_RADIUS * UNIT_TRIANGLE_ANGLE.cos() * UNIT_TRIANGLE_ANGLE.cos(),
            }),
        ),
    ])));

    commands.insert_resource(Materials(HashMap::from([
        (
            "red",
            materials.add(StandardMaterial {
                base_color: Color::hsl(RED_HUE, SATURATION, BRIGHTNESS),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "dark_red",
            materials.add(StandardMaterial {
                base_color: Color::hsl(RED_HUE, SATURATION, BRIGHTNESS / 2.),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "green",
            materials.add(StandardMaterial {
                base_color: Color::hsl(GREEN_HUE, SATURATION, BRIGHTNESS),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "green_trans",
            materials.add(StandardMaterial {
                base_color: Color::hsla(GREEN_HUE, SATURATION, BRIGHTNESS, 0.3),
                unlit: UNLIT,
                alpha_mode: AlphaMode::Blend,
                ..default()
            }),
        ),
        (
            "dark_green",
            materials.add(StandardMaterial {
                base_color: Color::hsl(GREEN_HUE, SATURATION, BRIGHTNESS / 2.),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "blue",
            materials.add(StandardMaterial {
                base_color: Color::hsl(BLUE_HUE, SATURATION, BRIGHTNESS),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "dark_blue",
            materials.add(StandardMaterial {
                base_color: Color::hsl(BLUE_HUE, SATURATION, BRIGHTNESS / 2.),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "yellow",
            materials.add(StandardMaterial {
                base_color: Color::hsl(YELLOW_HUE, SATURATION, BRIGHTNESS),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "teal",
            materials.add(StandardMaterial {
                base_color: Color::hsl(TEAL_HUE, SATURATION, BRIGHTNESS),
                unlit: UNLIT,
                ..default()
            }),
        ),
        (
            "purple",
            materials.add(StandardMaterial {
                base_color: Color::hsl(PURPLE_HUE, SATURATION, BRIGHTNESS),
                unlit: UNLIT,
                ..default()
            }),
        ),
    ])));
}

fn handle_mesh_requests(
    mut commands: Commands,
    meshes: Res<Meshes>,
    mut query: Query<(Entity, &mut Handle<Mesh>, &mut RequestMesh)>,
) {
    for (entity, mut mesh_2d_handle, request_mesh) in &mut query {
        if let Some(handle) = meshes.0.get(request_mesh.0) {
            *mesh_2d_handle = handle.clone();
        }
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<RequestMesh>();
        }
    }
}

fn handle_material_requests(
    mut commands: Commands,
    materials: Res<Materials>,
    mut query: Query<(Entity, &mut Handle<StandardMaterial>, &mut RequestMaterial)>,
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

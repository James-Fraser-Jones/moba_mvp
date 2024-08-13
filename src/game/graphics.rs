//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use super::{logic::types::*, *};
use bevy::{color::palettes::css, pbr::wireframe::Wireframe, prelude::*, render::*};
use cameras::orbit_camera::OrbitDistance;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;
use std::sync::LazyLock;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ColoredMaterialMap>();
        app.init_resource::<AllowedMeshMap>();
        app.add_systems(Startup, init);
        app.add_systems(Update, (add_entities, draw_cursor, anchor_nodes));
    }
}

pub const WALL_HEIGHT: f32 = 30.;
pub const BLENDER_WALL_HEIGHT: f32 = 50.;

#[derive(Component, Default)]
pub struct Map;

//HandleMap keys
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum OrderedMeshType {
    Capsule,
    Cylinder,
    Cuboid,
}
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct OrderedMesh {
    mesh_type: OrderedMeshType,
    radius: OrderedFloat<f32>,
    height: OrderedFloat<f32>,
}
impl Into<Mesh> for OrderedMesh {
    fn into(self) -> Mesh {
        match self.mesh_type {
            OrderedMeshType::Capsule => {
                Capsule3d::new(self.radius.0, self.height.0 - self.radius.0 * 2.).into()
            }
            OrderedMeshType::Cylinder => Cylinder::new(self.radius.0, self.height.0).into(),
            OrderedMeshType::Cuboid => {
                Cuboid::new(self.radius.0 * 2., self.radius.0 * 2., self.height.0).into()
            }
        }
    }
}
impl OrderedMesh {
    pub fn new(mesh_type: OrderedMeshType, radius: f32, height: f32) -> Self {
        Self {
            mesh_type,
            radius: OrderedFloat(radius),
            height: OrderedFloat(height),
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

#[derive(Resource)]
struct DevTexture(Handle<Image>);

//HandleMap wrappers for Meshes and Materials
#[derive(Resource, Default)]
struct ColoredMaterialMap(os::HandleMap<OrderedColor, StandardMaterial>);
impl ColoredMaterialMap {
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
#[derive(Resource, Default)]
struct AllowedMeshMap(os::HandleMap<OrderedMesh, Mesh>);
impl AllowedMeshMap {
    fn clone_mesh_handle(
        &mut self,
        meshes: &mut Assets<Mesh>,
        allowed_mesh: OrderedMesh,
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
pub struct RenderComponent {
    pub color: Color,

    pub mesh_type: OrderedMeshType,
    pub height: f32,

    pub wireframe: bool,
    pub raised: bool,
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
    commands.insert_resource(DevTexture(
        server.load("textures/untracked/kenney_dev_textures/Light/texture_07.png"),
    ));
}

fn add_entities(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Radius,
            &mut RenderComponent,
            Option<&Health>,
            Option<&Team>,
        ),
        Added<RenderComponent>,
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_map: ResMut<ColoredMaterialMap>,
    mut mesh_map: ResMut<AllowedMeshMap>,
    dev_texture: Res<DevTexture>,
) {
    for (entity, radius, display, health, team) in &mut query {
        let ordered_mesh = OrderedMesh::new(display.mesh_type, radius.0, display.height);
        let child = PbrBundle {
            mesh: mesh_map.clone_mesh_handle(&mut meshes, ordered_mesh),
            material: material_map.clone_material_handle(
                &mut materials,
                display.color,
                Some(&dev_texture.0),
            ),
            transform: Transform::from_translation(Vec3::ZERO.with_z(if display.raised {
                display.height / 2.
            } else {
                0.
            }))
            .with_rotation(Quat::from_rotation_x(
                if display.mesh_type == OrderedMeshType::Cuboid {
                    0.
                } else {
                    PI / 2.
                },
            )),
            ..default()
        };
        if display.wireframe {
            let child = commands.spawn((child, Wireframe)).id();
            commands.entity(entity).add_child(child);
        } else {
            let child = commands.spawn(child).id();
            commands.entity(entity).add_child(child);
        }
        if let Some(_) = health {
            //advanced health bar
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            padding: UiRect::all(Val::Px(2.)),
                            column_gap: Val::Px(2.),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    },
                    DisplayUIAnchor(entity),
                ))
                .with_children(|builder| {
                    builder.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(team_color(team.copied())),
                        ..default()
                    });
                    builder.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(team_color(team.copied())),
                        ..default()
                    });
                    builder.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(100.),
                            ..default()
                        },
                        background_color: BackgroundColor(team_color(team.copied())),
                        ..default()
                    });
                    builder.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(72.),
                            ..default()
                        },
                        background_color: BackgroundColor(team_color(team.copied())),
                        ..default()
                    });
                    builder.spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            width: Val::Percent(300.), //need to ensure this value accounts for missing gap pixels to prevent distortion
                            ..default()
                        },
                        background_color: BackgroundColor(Color::BLACK),
                        ..default()
                    });
                    builder.spawn(
                        TextBundle::from_section(
                            "372",
                            TextStyle {
                                font_size: 36.,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            position_type: PositionType::Absolute,
                            height: Val::Percent(100.),
                            width: Val::Percent(100.),
                            ..default()
                        })
                        .with_text_justify(JustifyText::Center),
                    );
                });

            // //basic health bar
            // commands
            //     .spawn((
            //         NodeBundle {
            //             style: Style {
            //                 position_type: PositionType::Absolute,
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(Color::BLACK),
            //             ..default()
            //         },
            //         DisplayUIAnchor(entity),
            //     ))
            //     .with_children(|builder| {
            //         builder.spawn(NodeBundle {
            //             style: Style {
            //                 width: Val::Percent(50.),
            //                 height: Val::Percent(100.),
            //                 ..default()
            //             },
            //             background_color: BackgroundColor(team_color(team.copied())),
            //             ..default()
            //         });
            //     });
        }
    }
}

pub const RED_TEAM_COLOR: Color = Color::Srgba(css::TOMATO);
pub const BLUE_TEAM_COLOR: Color = Color::Srgba(css::DEEP_SKY_BLUE);
pub const NO_TEAM_COLOR: Color = Color::Srgba(css::LIGHT_GREEN);
pub fn team_color(team: Option<Team>) -> Color {
    match team {
        Some(team) => match team {
            Team::Red => RED_TEAM_COLOR,
            Team::Blue => BLUE_TEAM_COLOR,
        },
        None => NO_TEAM_COLOR,
    }
}

//logical pixels, top-left (0,0), to Vec2 representing intersection point with horizontal plane of height, in world space
fn pixel_to_horizontal_plane(
    pixel: Vec2,
    height: f32,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let pixel_ray = camera.viewport_to_world(camera_transform, pixel).unwrap();
    let intersection_distance =
        pixel_ray.intersect_plane(Vec3::Z * height, InfinitePlane3d::new(Vec3::Z))?;
    let intersection_point = pixel_ray.get_point(intersection_distance);
    Some(intersection_point.truncate())
}

fn position_to_pixel(
    position: Vec3,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    camera.world_to_viewport(camera_transform, position)
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform), With<cameras::orbit_camera::OrbitDistance>>,
    last_cursor_position: Res<input::LastCursorPosition>,
    mut gizmos: Gizmos,
) {
    let ground_plane_height = 0.;
    let (camera, camera_transform) = camera_query.single();
    if let Some(point) = pixel_to_horizontal_plane(
        last_cursor_position.0,
        ground_plane_height,
        &camera,
        &camera_transform,
    ) {
        gizmos.circle(
            point.extend(0.01),
            Dir3::new(Vec3::Z).unwrap(),
            10.,
            Color::WHITE,
        );
        gizmos.arrow(point.extend(30.), point.extend(0.01), Color::WHITE);
    }
}

fn anchor_nodes(
    mut anchor_query: Query<(&mut Style, &DisplayUIAnchor)>,
    display_query: Query<(&Transform, &RenderComponent, &Radius)>,
    camera_query: Query<(&Camera, &GlobalTransform, &Transform), With<OrbitDistance>>,
) {
    let (camera, global_transform, camera_transform) = camera_query.single();
    for (mut style, anchor) in &mut anchor_query {
        let (transform, display, radius) = display_query.get(anchor.0).unwrap();
        let mut elevation = display.height;
        if !display.raised {
            elevation /= 2.;
        }
        let ang = camera_transform.rotation.to_euler(EulerRot::ZYX).2;
        let anchor_point = transform.translation
            + Vec3::new(
                0.,
                radius.0 * ang.cos() + HEALTH_BAR_OFFSET,
                elevation * ang.sin() + HEALTH_BAR_OFFSET,
            );
        if let Some(pixel_point) = position_to_pixel(anchor_point, camera, global_transform) {
            let distance_from_camera = (camera_transform.translation - anchor_point).length();
            let intended_width = HEALTH_BAR_WIDTH * radius.0;
            let size = Vec2::new(intended_width, intended_width / HEALTH_BAR_ASPECT_RATIO)
                / distance_from_camera;
            style.width = Val::Px(size.x);
            style.height = Val::Px(size.y);
            style.left = Val::Px(pixel_point.x - size.x / 2.);
            style.top = Val::Px(pixel_point.y - size.y);
        }
    }
}

const HEALTH_BAR_ASPECT_RATIO: f32 = 10.;
const HEALTH_BAR_WIDTH: f32 = 2700.;
const HEALTH_BAR_OFFSET: f32 = 5.;

#[derive(Component)]
struct DisplayUIAnchor(Entity);

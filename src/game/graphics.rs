//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

pub mod healthbar;
pub mod map;
pub mod model;

use super::{logic::types::*, *};
use bevy::{color::palettes::css, pbr::wireframe::Wireframe, prelude::*, render::*};
use cameras::orbit_camera::OrbitDistance;
use ordered_float::OrderedFloat;
use std::f32::consts::PI;
use std::sync::LazyLock;

pub const RED_TEAM_COLOR: Color = Color::Srgba(css::TOMATO);
pub const BLUE_TEAM_COLOR: Color = Color::Srgba(css::DEEP_SKY_BLUE);
pub const NO_TEAM_COLOR: Color = Color::Srgba(css::LIGHT_GREEN);

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(model::ModelPlugin);
        app.add_systems(Startup, (init, map::init));
        app.add_systems(
            Update,
            (
                draw_cursor,
                map::update,
                healthbar::add_healthbars,
                healthbar::anchor_healthbars,
            ),
        );
    }
}

fn init() {}

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

pub fn team_color(team: Option<Team>) -> Color {
    match team {
        Some(team) => match team {
            Team::Red => RED_TEAM_COLOR,
            Team::Blue => BLUE_TEAM_COLOR,
        },
        None => NO_TEAM_COLOR,
    }
}

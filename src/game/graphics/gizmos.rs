//responsibilities:
//enabling wireframe, gizmos, etc..
//stuff for purely-dev-related functionality

use super::super::{types::*, *};
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;

pub struct GizmosPlugin;
impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WireframePlugin, FpsOverlayPlugin::default()));
        app.add_systems(Startup, init);
        app.add_systems(
            Update,
            (draw_player, draw_wireframe, draw_cursor3d).in_set(UpdateGraphics),
        );
    }
}

const WIREFRAME_ENABLED: bool = false;

fn init(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = WIREFRAME_ENABLED;
}

fn draw_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn draw_player(
    mut gizmos: Gizmos,
    player: Res<player::Player>,
    player_query: Query<(&Transform, &MovePosition)>,
) {
    let (transform, move_position) = player_query.get(player.0).unwrap();
    let start = transform.translation.with_z(25.);
    let diff = transform.rotation.mul_vec3(Vec3::X * 30.);
    gizmos.arrow(start, start + diff, Color::WHITE);
    if let Some(goal) = move_position.0 {
        gizmos.line(
            transform.translation.with_z(0.01),
            goal.extend(0.01),
            Color::WHITE,
        );
    }
}

fn draw_cursor3d(
    mut gizmos: Gizmos,
    camera_query: Query<(&Camera, &GlobalTransform), With<cameras::orbit_camera::OrbitDistance>>,
    cursor_2d: Res<input::Cursor2D>,
) {
    let (camera, transform) = camera_query.single();
    if let Some(point) =
        cameras::orbit_camera::pixel_to_horizontal_plane(cursor_2d.0, 0., camera, &transform)
    {
        gizmos.circle(
            point.extend(0.01),
            Dir3::new(Vec3::Z).unwrap(),
            10.,
            Color::WHITE,
        );
        gizmos.arrow(point.extend(30.), point.extend(0.01), Color::WHITE);
    }
}

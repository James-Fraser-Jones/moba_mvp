//responsibilities:
//enabling wireframe, gizmos, etc..
//stuff for purely-dev-related functionality

use super::{types::*, *};
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;

pub struct DevPlugin;
impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WireframePlugin, FpsOverlayPlugin::default()));
        app.add_systems(Startup, init);
        app.add_systems(Update, (draw_player, draw_wireframe));
        app.add_systems(
            PostUpdate,
            draw_cursor.after(input::get_cursor_world_position),
        );
    }
}

const WIREFRAME_ENABLED: bool = false;

fn init(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = WIREFRAME_ENABLED;
}

fn draw_player(
    mut gizmos: Gizmos,
    player: Res<player::Player>,
    query: Query<(&Transform, &MovePosition)>,
) {
    let (transform, move_position) = query.get(player.0).unwrap();
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

fn draw_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn draw_cursor(cursor_world_position: Res<input::CursorWorldPosition>, mut gizmos: Gizmos) {
    if let Some(point) = cursor_world_position.0 {
        gizmos.circle(
            point.extend(0.01),
            Dir3::new(Vec3::Z).unwrap(),
            10.,
            Color::WHITE,
        );
        gizmos.arrow(point.extend(30.), point.extend(0.01), Color::WHITE);
    }
}

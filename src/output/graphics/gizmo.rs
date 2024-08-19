use crate::*;
use bevy::{pbr::wireframe::WireframeConfig, prelude::*};

pub struct GizmoPlugin;
impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(GizmoSet));
        app.add_systems(
            Update,
            (update_player, update_wireframe, update_cursor3d).in_set(GizmoSet),
        );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct GizmoSet;

const WIREFRAME_ENABLED: bool = false;

fn init(mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = WIREFRAME_ENABLED;
}

fn update_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyV) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn update_player(
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

fn update_cursor3d(
    mut gizmos: Gizmos,
    camera_query: Query<(&Camera, &GlobalTransform), With<OrbitDistance>>,
    cursor_2d: Res<input::CursorPosition2D>,
) {
    let (camera, transform) = camera_query.single();
    if let Some(point) = pixel_to_horizontal_plane(cursor_2d.0, 0., camera, &transform) {
        gizmos.circle(
            point.extend(0.01),
            Dir3::new(Vec3::Z).unwrap(),
            10.,
            Color::WHITE,
        );
        gizmos.arrow(point.extend(30.), point.extend(0.01), Color::WHITE);
    }
}

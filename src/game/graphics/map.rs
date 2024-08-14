use super::*;
use bevy::prelude::*;

pub const WALL_HEIGHT: f32 = 30.;
pub const BLENDER_WALL_HEIGHT: f32 = 50.;

#[derive(Component, Default)]
pub struct Map;

pub fn init(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: server.load("models/map.glb#Scene0"),
            transform: Transform::from_scale(Vec3::ONE.with_z(WALL_HEIGHT / BLENDER_WALL_HEIGHT)),
            ..default()
        },
        Map,
    ));
}

pub fn update(
    mut map_query: Query<&mut Transform, With<Map>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        let mut transform = map_query.single_mut();
        transform.scale.z += 10. / BLENDER_WALL_HEIGHT;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        let mut transform = map_query.single_mut();
        transform.scale.z -= 10. / BLENDER_WALL_HEIGHT;
    }
    if keyboard.just_pressed(KeyCode::KeyR) {
        let mut transform = map_query.single_mut();
        transform.scale.z = WALL_HEIGHT / BLENDER_WALL_HEIGHT;
    }
}

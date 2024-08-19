//use crate::game::*;
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(MapSet));
        app.add_systems(Update, update.in_set(MapSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapSet;

pub const BLENDER_LANE_WIDTH: f32 = 130.;
pub const BLENDER_OUTER_WALL_WIDTH: f32 = 100.;
pub const BLENDER_WALL_HEIGHT: f32 = 50.;
const DESIRED_WALL_HEIGHT: f32 = 30.;

#[derive(Component, Default)]
struct Map;

fn init(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: server.load("models/map.glb#Scene0"),
            transform: Transform::from_scale(
                Vec3::ONE.with_z(DESIRED_WALL_HEIGHT / BLENDER_WALL_HEIGHT),
            ),
            ..default()
        },
        Map,
    ));
}

fn update(mut map_query: Query<&mut Transform, With<Map>>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        let mut transform = map_query.single_mut();
        transform.scale.z += 10. / BLENDER_WALL_HEIGHT;
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        let mut transform = map_query.single_mut();
        transform.scale.z -= 10. / BLENDER_WALL_HEIGHT;
    }
}

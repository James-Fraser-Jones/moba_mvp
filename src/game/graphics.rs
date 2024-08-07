//responsibilities:
//initializing meshes/materials correctly
//adding/removing meshes/materials to/from the world, mostly in accordance with entities added/removed by the logic plugin

use crate::game::{os::Handles, *};
use bevy::color::palettes::css;
use bevy::gltf::GltfMesh;
use bevy::{math::Affine2, prelude::*, render::*};
use std::f32::consts::PI;

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    //Approx 50% of the time this results in draw errors
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: material_assets.add(StandardMaterial {
            base_color: Color::Srgba(css::ROYAL_BLUE),
            unlit: true,
            ..default()
        }),
        transform: Transform::from_translation(Vec3::new(0., 0., 200.)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: material_assets.add(StandardMaterial {
            base_color: Color::Srgba(css::DARK_RED),
            unlit: true,
            double_sided: false,
            cull_mode: None,
            ..default()
        }),
        ..default()
    });
}

use bevy::{math::Affine2, prelude::*, render::*};

pub struct GraphicsPlugin;
impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    let texture = server.load_with_settings(
        "textures/kenney_dev_textures/Dark/texture_07.png",
        |settings: &mut texture::ImageLoaderSettings| {
            settings.sampler = texture::ImageSampler::Descriptor(texture::ImageSamplerDescriptor {
                address_mode_u: texture::ImageAddressMode::Repeat,
                address_mode_v: texture::ImageAddressMode::Repeat,
                ..default()
            })
        },
    );

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(1000.))),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            unlit: true,
            uv_transform: Affine2::from_scale(Vec2::splat(10.)),
            ..default()
        }),
        ..default()
    });
}

fn update() {}

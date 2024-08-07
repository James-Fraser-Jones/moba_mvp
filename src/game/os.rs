//responsibilities:
//window settings and utils
//quitting the game
//saving/loading assets to/from the filesystem

use bevy::{prelude::*, render::*, utils::hashbrown::HashMap, window::*};

pub struct OSPlugin;
impl Plugin for OSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_resources, init).chain());
        app.add_systems(Update, (sync_window, exit_game));
    }
}

#[derive(Resource, Default)]
pub struct Handles<A: Asset>(pub HashMap<String, Handle<A>>);
impl<A: Asset> Handles<A> {
    pub fn add_value(&mut self, assets: &mut Assets<A>, name: &str, value: impl Into<A>) {
        self.0.insert(name.to_string(), assets.add(value));
    }
    pub fn add_path(&mut self, server: &AssetServer, path: &str) {
        self.0
            .insert(path.to_string(), server.load(path.to_string()));
    }
    // pub fn add_handle(&mut self, assets: &mut Assets<A>, name: &str) {
    //     self.0.insert(name.to_string(), assets.reserve_handle());
    // }
    pub fn get_handle(&self, name: &str) -> &Handle<A> {
        self.0.get(name).unwrap()
    }
    pub fn get_value<'a>(&self, assets: &'a mut Assets<A>, name: &str) -> &'a A {
        assets.get(self.get_handle(name)).unwrap()
    }
    pub fn get_value_mut<'a>(&self, assets: &'a mut Assets<A>, name: &str) -> &'a mut A {
        assets.get_mut(self.get_handle(name)).unwrap()
    }
}

fn init_resources(mut commands: Commands) {
    commands.init_resource::<MainWindow>();
    commands.insert_resource(Handles::<Gltf>(HashMap::default()));
    commands.init_resource::<Handles<Image>>();
    commands.init_resource::<Handles<StandardMaterial>>();
    commands.insert_resource(Handles::<Mesh>(HashMap::default()));
}

pub fn init(
    main_window: Res<MainWindow>,
    server: Res<AssetServer>,
    mut window_query: Query<&mut Window>,
    mut gltf_handles: ResMut<Handles<Gltf>>,
    mut image_handles: ResMut<Handles<Image>>,
) {
    //sync resource with entity
    let mut window = window_query.single_mut();
    *window = main_window.0.clone();

    //load assets from file system
    for gltf_path in ["models/map.glb"] {
        gltf_handles.add_path(&server, &gltf_path);
    }
    for image_path in [
        "textures/kenney_dev_textures/Dark/texture_07.png",
        "textures/kenney_dev_textures/Orange/texture_08.png",
        "textures/kenney_dev_textures/Green/texture_08.png",
    ] {
        image_handles.0.insert(
            image_path.to_string(),
            server.load_with_settings(image_path, |settings: &mut texture::ImageLoaderSettings| {
                settings.sampler =
                    texture::ImageSampler::Descriptor(texture::ImageSamplerDescriptor {
                        address_mode_u: texture::ImageAddressMode::Repeat,
                        address_mode_v: texture::ImageAddressMode::Repeat,
                        ..default()
                    })
            }),
        );
    }
}

fn sync_window(mut main_window: ResMut<MainWindow>, mut window_query: Query<&mut Window>) {
    let mut window = window_query.single_mut();
    if window.is_changed() {
        main_window.0 = window.clone();
    } else if main_window.is_changed() {
        *window = main_window.0.clone();
    }
}

fn exit_game(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

#[derive(Resource)]
pub struct MainWindow(Window);
impl Default for MainWindow {
    fn default() -> Self {
        let game_name = "Moba MVP";
        Self(Window {
            title: game_name.to_string(),
            name: Some(game_name.to_string()),
            position: WindowPosition::At(IVec2::new(0, 0)),
            resolution: WindowResolution::new(1920., 1080.),
            mode: WindowMode::Windowed,
            cursor: Cursor {
                grab_mode: CursorGrabMode::None,
                ..default()
            },
            ..default()
        })
    }
}
impl MainWindow {
    pub fn aspect_ratio(&self) -> f32 {
        let size = self.0.resolution.size();
        size.x / size.y
    }
}

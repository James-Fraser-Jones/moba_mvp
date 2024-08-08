//responsibilities:
//window settings and utils
//quitting the game
//saving/loading assets to/from the filesystem

use bevy::{prelude::*, utils::hashbrown::HashMap, window::*};
use std::sync::LazyLock;

pub struct OSPlugin;
impl Plugin for OSPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Handles<StandardMaterial>>();
        app.insert_resource(Handles::<Mesh>(HashMap::default()));
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

#[derive(Resource, Default)]
pub struct Handles<A: Asset>(HashMap<String, Handle<A>>);
impl<A: Asset> Handles<A> {
    pub fn add_handle(&mut self, name: &str, handle: Handle<A>) {
        self.0.insert(name.to_string(), handle);
    }
    pub fn add_asset(&mut self, assets: &mut Assets<A>, name: &str, value: impl Into<A>) {
        self.add_handle(name, assets.add(value));
    }
    pub fn add_path(&mut self, server: &AssetServer, name: &str, path: &str) {
        self.add_handle(name, server.load(path.to_string()));
    }
    // pub fn reserve_handle(&mut self, assets: &mut Assets<A>, name: &str) {
    //     self.0.insert(name.to_string(), assets.reserve_handle());
    // }
    pub fn get_handle(&self, name: &str) -> &Handle<A> {
        self.0.get(name).unwrap()
    }
    pub fn get_asset<'a>(&self, assets: &'a mut Assets<A>, name: &str) -> &'a A {
        assets.get(self.get_handle(name)).unwrap()
    }
    pub fn get_asset_mut<'a>(&self, assets: &'a mut Assets<A>, name: &str) -> &'a mut A {
        assets.get_mut(self.get_handle(name)).unwrap()
    }
}

const GAME_NAME: &str = "Moba MVP";
static WINDOW: LazyLock<Window> = LazyLock::new(|| Window {
    title: GAME_NAME.to_string(),
    name: Some(GAME_NAME.to_string()),
    position: WindowPosition::At(IVec2::new(0, 0)),
    resolution: WindowResolution::new(1920., 1080.),
    mode: WindowMode::Windowed,
    cursor: Cursor {
        grab_mode: CursorGrabMode::None,
        ..default()
    },
    ..default()
});

pub fn aspect_ratio(window: &Window) -> f32 {
    let size = window.resolution.size();
    size.x / size.y
}

fn init(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    *window = WINDOW.clone();
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

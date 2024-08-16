//responsibilities:
//window settings and utils
//quitting the game
//saving/loading assets to/from the filesystem

use bevy::{prelude::*, window::*, winit::WinitWindows};
use std::collections::HashMap;
use std::sync::LazyLock;
use winit::window::Icon;

pub struct OSPlugin;
impl Plugin for OSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

const GAME_ICON: &str = "assets/textures/moba_icon.png";
const GAME_NAME: &str = "Moba MVP";
static WINDOW: LazyLock<Window> = LazyLock::new(|| Window {
    title: GAME_NAME.to_string(),
    name: Some(GAME_NAME.to_string()),
    position: WindowPosition::At(IVec2::new(0, 0)),
    resolution: WindowResolution::new(1920., 1080.),
    mode: WindowMode::Windowed,
    cursor: Cursor {
        grab_mode: CursorGrabMode::None,
        visible: false,
        ..default()
    },
    ..default()
});

#[derive(Resource)]
pub struct HandleMap<K: Eq + std::hash::Hash, A: Asset>(pub HashMap<K, Handle<A>>);
impl<K: Eq + std::hash::Hash, A: Asset> HandleMap<K, A> {
    pub fn insert_asset(
        &mut self,
        assets: &mut Assets<A>,
        key: K,
        value: impl Into<A>,
    ) -> &Handle<A> {
        let handle = assets.add(value);
        self.0.entry(key).or_insert(handle)
    }
    pub fn insert_asset_path(&mut self, server: &AssetServer, key: K, path: &str) -> &Handle<A> {
        let handle = server.load(path.to_string());
        self.0.entry(key).or_insert(handle)
    }
    pub fn get_asset<'a>(&self, assets: &'a mut Assets<A>, key: &K) -> Option<&'a A> {
        let handle = self.0.get(key)?;
        assets.get(handle)
    }
    pub fn get_asset_mut<'a>(&self, assets: &'a mut Assets<A>, key: &K) -> Option<&'a mut A> {
        let handle = self.0.get(key)?;
        assets.get_mut(handle)
    }
}
impl<K: Eq + std::hash::Hash, A: Asset> Default for HandleMap<K, A> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

fn init(mut window_query: Query<&mut Window, With<PrimaryWindow>>, windows: NonSend<WinitWindows>) {
    //set window settings
    let mut window = window_query.single_mut();
    *window = WINDOW.clone();
    //set window icon
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(GAME_ICON)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

//responsibilities:
//window settings and utils
//quitting the game
//saving/loading assets to/from the filesystem

use bevy::{prelude::*, window::*};
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct OSPlugin;
impl Plugin for OSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

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

fn init(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    *window = WINDOW.clone();
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

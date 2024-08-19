//responsibilities:
//window settings and utils
//quitting the game
//saving/loading assets to/from the filesystem

//use crate::game::*;
use bevy::{prelude::*, window::*, winit::WinitWindows};
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
static GAME_WINDOW: LazyLock<Window> = LazyLock::new(|| Window {
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

fn init(mut window_query: Query<&mut Window, With<PrimaryWindow>>, windows: NonSend<WinitWindows>) {
    //set window settings
    let mut window = window_query.single_mut();
    *window = GAME_WINDOW.clone();
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

fn update() {}

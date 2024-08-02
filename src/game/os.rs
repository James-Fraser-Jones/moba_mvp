use bevy::{prelude::*, window::*};

pub struct OSPlugin;
impl Plugin for OSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init(mut commands: Commands, mut query: Query<&mut Window>) {
    let window_settings = WindowSettings::default();
    let mut primary_window = query.get_single_mut().unwrap();
    set_window(&window_settings, &mut primary_window);
    commands.insert_resource(window_settings);
}

fn update(
    window_settings: Res<WindowSettings>,
    mut query: Query<&mut Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<AppExit>,
) {
    if window_settings.is_changed() {
        let mut primary_window = query.get_single_mut().unwrap();
        set_window(&window_settings, &mut primary_window);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

#[derive(Resource)]
pub struct WindowSettings {
    pub name: &'static str,
    pub position: IVec2,
    pub size: Vec2,
    pub mode: WindowMode,
    pub cursor_grab: CursorGrabMode,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            name: "Moba MVP",
            position: IVec2::new(0, 0),
            size: Vec2::new(960., 540.),
            mode: WindowMode::Windowed, //mode: WindowMode::BorderlessFullscreen,
            cursor_grab: CursorGrabMode::None, //cursor_grab: CursorGrabMode::Confined,
        }
    }
}
impl WindowSettings {
    pub fn aspect_ratio(&self) -> f32 {
        self.size.x / self.size.y
    }
}

fn set_window(game_window: &WindowSettings, window: &mut Window) {
    *window = Window {
        title: game_window.name.into(),
        name: Some(game_window.name.into()),
        position: WindowPosition::new(game_window.position),
        resolution: WindowResolution::new(game_window.size.x, game_window.size.y),
        mode: game_window.mode,
        cursor: Cursor {
            grab_mode: game_window.cursor_grab,
            visible: true,
            ..default()
        },
        ..default()
    };
}

use bevy::{prelude::*, window::*};

pub struct OSPlugin;
impl Plugin for OSPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

pub fn init(mut commands: Commands, mut query: Query<&mut Window>) {
    let window_settings = WindowSettings::default();
    let mut primary_window = query.get_single_mut().unwrap();
    settings_to_window(&window_settings, &mut primary_window);
    commands.insert_resource(window_settings);
}

fn update(
    mut window_settings: ResMut<WindowSettings>,
    mut window_query: Query<&mut Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<AppExit>,
) {
    let mut window = window_query.single_mut();
    if window.is_changed() {
        window_to_settings(&window, &mut window_settings);
    } else if window_settings.is_changed() {
        settings_to_window(&window_settings, &mut window);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

#[derive(Resource)]
pub struct WindowSettings {
    pub name: String,
    pub position: IVec2,
    pub size: Vec2,
    pub mode: WindowMode,
    pub cursor_grab: CursorGrabMode,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            name: "Moba MVP".to_string(),
            position: IVec2::new(0, 0),
            size: Vec2::new(1920., 1080.),
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

fn settings_to_window(window_settings: &WindowSettings, window: &mut Window) {
    window.title = window_settings.name.clone();
    window.name = Some(window_settings.name.clone());
    window.position = WindowPosition::new(window_settings.position);
    window.resolution = WindowResolution::new(window_settings.size.x, window_settings.size.y);
    window.mode = window_settings.mode;
    window.cursor.grab_mode = window_settings.cursor_grab;
}

fn window_to_settings(window: &Window, window_settings: &mut WindowSettings) {
    window_settings.name = window.title.clone();
    if let WindowPosition::At(pos) = window.position {
        window_settings.position = pos;
    };
    window_settings.size = window.resolution.size();
    window_settings.mode = window.mode;
    window_settings.cursor_grab = window.cursor.grab_mode;
}

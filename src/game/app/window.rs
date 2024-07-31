use bevy::{prelude::*, window::*};

pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init(mut commands: Commands, mut query: Query<&mut Window>) {
    let game_window = WindowSettings::default();
    for mut window in &mut query {
        set_window(&game_window, &mut window);
    }
    commands.init_resource::<WindowSettings>();
}

fn update() {}

#[derive(Resource)] //read-only
pub struct WindowSettings {
    name: &'static str,
    position: IVec2,
    size: Vec2,
    mode: WindowMode,
    cursor_grab: CursorGrabMode,
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self {
            name: "Moba MVP",
            position: IVec2::new(0, 0),
            size: Vec2::new(960., 540.),
            mode: WindowMode::Windowed,
            //mode: WindowMode::BorderlessFullscreen,
            cursor_grab: CursorGrabMode::None,
            //cursor_grab: CursorGrabMode::Confined,
        }
    }
}
impl WindowSettings {
    fn aspect_ratio(&self) -> f32 {
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

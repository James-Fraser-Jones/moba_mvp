pub mod input;
pub use input::*;
pub mod process;
pub use process::*;
pub mod output;
pub use output::*;

use bevy::{
    app::PluginGroupBuilder, dev_tools::fps_overlay::FpsOverlayPlugin,
    pbr::wireframe::WireframePlugin, prelude::*, window::*, winit::WinitWindows,
};
use bevy_framepace::*;
use std::sync::LazyLock;
use winit::window::Icon;

fn main() -> AppExit {
    App::new().add_plugins(MobaPlugin).run()
}

pub struct MobaPlugin;
impl Plugin for MobaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(MainSet));
        app.add_systems(Update, update.in_set(MainSet));
        //plugins
        app.add_plugins(DefaultPlugins);
        app.add_plugins(AdditionalPlugins);
        app.add_plugins(ExternalPlugins);
        app.add_plugins(GamePlugins);
        //core ordering configuration
        app.configure_sets(PreUpdate, InputSet.after(bevy::input::InputSystem));
        app.configure_sets(PreUpdate, ActionSet.after(AxisSet));
        app.configure_sets(FixedUpdate, (PlayerSet, LogicSet, PhysicsSet).chain());
        //specific ordering dependencies
        app.configure_sets(Startup, PlayerSet.after(LogicSet));
        app.configure_sets(Update, (GizmoSet, HealthbarSet).after(OrbitCameraSet));
        //utility system sets
        app.configure_sets(
            PostUpdate,
            AfterTransformPropagationSet.after(TransformSystem::TransformPropagate),
        );
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
const UPDATE_MAX_RATE: Option<f64> = None;
const UPDATE_FIXED_RATE: f64 = 16.;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MainSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AfterTransformPropagationSet;

pub struct ExternalPlugins;
impl PluginGroup for ExternalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(FramepacePlugin)
    }
}

pub struct AdditionalPlugins;
impl PluginGroup for AdditionalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FpsOverlayPlugin::default())
            .add(WireframePlugin)
    }
}

pub struct GamePlugins;
impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            //preupdate
            .add(AxisPlugin)
            .add(ActionPlugin)
            //fixedupdate
            .add(PlayerPlugin)
            .add(LogicPlugin)
            .add(PhysicsPlugin)
            //update
            .add(OrbitCameraPlugin)
            .add(OverlayCameraPlugin)
            .add(MapPlugin)
            .add(ModelPlugin)
            .add(HealthbarPlugin)
            .add(GizmoPlugin)
    }
}

fn init(
    mut settings: ResMut<FramepaceSettings>,
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    //set window settings
    let mut window = window_query.single_mut();
    *window = GAME_WINDOW.clone();
    //set window icon
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(GAME_ICON).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
    //set update rate limit
    if let Some(max_rate) = UPDATE_MAX_RATE {
        settings.limiter = Limiter::from_framerate(max_rate);
    }
    //set fixed-update rate
    commands.insert_resource(Time::<Fixed>::from_hz(UPDATE_FIXED_RATE));
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    //exit game
    if keyboard.pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

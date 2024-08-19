//use crate::game::*;
use bevy::prelude::*;
use bevy_framepace::*;

pub struct SystemPlugin;
impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin);
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
        app.configure_sets(PreUpdate, InputSet.after(bevy::input::InputSystem));
        app.configure_sets(FixedUpdate, (PlayerSet, LogicSet, PhysicsSet).chain());
        app.configure_sets(
            PostUpdate,
            ProjectCameraSet.after(TransformSystem::TransformPropagate),
        );
    }
}

const UPDATE_MAX_RATE: f64 = 60.;
const UPDATE_FIXED_RATE: f64 = 60.;

fn init(mut settings: ResMut<FramepaceSettings>, mut commands: Commands) {
    settings.limiter = Limiter::from_framerate(UPDATE_MAX_RATE);
    commands.insert_resource(Time::<Fixed>::from_hz(UPDATE_FIXED_RATE));
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LogicSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProjectCameraSet;

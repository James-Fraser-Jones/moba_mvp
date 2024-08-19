use crate::game::*;
use bevy::prelude::*;
use bevy_framepace::*;

pub struct OrderPlugin;
impl Plugin for OrderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FramepacePlugin);
        app.add_systems(Startup, init.in_set(OrderSet));
        app.add_systems(Update, update.in_set(OrderSet));
        //core ordering configuration
        app.configure_sets(PreUpdate, InputSet.after(bevy::input::InputSystem));
        app.configure_sets(PreUpdate, ActionSet.after(AxisSet));
        app.configure_sets(FixedUpdate, (PlayerSet, LogicSet, PhysicsSet).chain());
        //specific ordering dependencies
        app.configure_sets(Startup, PlayerSet.after(LogicSet));
        app.configure_sets(Update, (GizmosSet, HealthbarSet).after(OrbitCameraSet));
        //utility sets
        app.configure_sets(
            PostUpdate,
            AfterTransformPropagationSet.after(TransformSystem::TransformPropagate),
        );
    }
}

const UPDATE_MAX_RATE: Option<f64> = Some(60.);
const UPDATE_FIXED_RATE: f64 = 1.;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AfterTransformPropagationSet;

fn init(mut settings: ResMut<FramepaceSettings>, mut commands: Commands) {
    if let Some(max_rate) = UPDATE_MAX_RATE {
        settings.limiter = Limiter::from_framerate(max_rate);
    }
    commands.insert_resource(Time::<Fixed>::from_hz(UPDATE_FIXED_RATE));
}

fn update(keyboard: Res<ButtonInput<KeyCode>>, mut writer: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

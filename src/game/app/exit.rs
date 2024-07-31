use bevy::prelude::*;

pub struct ExitPlugin;
impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(Update, update);
    }
}

fn init() {}

fn update() {}

fn exit(mut writer: EventWriter<AppExit>) {
    writer.send(AppExit::Success);
}

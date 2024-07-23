use bevy::prelude::*;

pub struct QuitPlugin;

impl Plugin for QuitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_quit);
    }
}

fn update_quit(mut writer: EventWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

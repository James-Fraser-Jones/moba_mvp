mod game;

use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, game::GamePlugins))
        .run()
}

mod game;

use bevy::pbr::wireframe::WireframePlugin;
use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WireframePlugin, game::GamePlugins))
        .run()
}

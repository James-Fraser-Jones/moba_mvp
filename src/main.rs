mod game;

use bevy::prelude::*;
use game::*;

fn main() -> AppExit {
    App::new().add_plugins((DefaultPlugins, GamePlugins)).run()
}

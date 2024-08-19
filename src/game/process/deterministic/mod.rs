pub mod logic;
pub use logic::*;
pub mod physics;
pub use physics::*;
pub mod spawn;
pub use spawn::*;

use bevy::prelude::*;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeterministicSet;

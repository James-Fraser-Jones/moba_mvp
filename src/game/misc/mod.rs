pub mod assets;
pub use assets::*;
pub mod os;
pub use os::*;

use bevy::prelude::*;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MiscSet;

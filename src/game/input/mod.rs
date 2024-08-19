pub mod axis;
pub use axis::*;
pub mod action;
pub use action::*;

use bevy::prelude::*;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

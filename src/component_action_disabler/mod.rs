//! This module implements helper components which can be used to disable ActionStates on the
//! entity they are attached to. Useful for pause menus, cutscenes, multiplayer, etc.
//!
//! All of these components will override any manually set [`ActionState::disabled`] on all
//! ActionStates for an entity. It is intended that you either use these disabling components, or
//! manually set [`ActionState::disabled`], not both combined.

mod unit;
mod relationship;

pub use unit::*;
pub use relationship::*;

use godot::prelude::*;

#[macro_use]
pub mod godot_utils;

pub mod app;
pub mod game_state;

struct DungeonStrandingExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DungeonStrandingExtension {}

use godot::prelude::*;

pub mod app;
pub mod game_state;

struct DungeonStrandingExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DungeonStrandingExtension {}

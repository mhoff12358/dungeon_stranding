use godot::prelude::*;

#[macro_use]
pub mod godot_utils;

pub mod app;
pub mod game_state;
pub mod my_gd_ref;
pub mod out_of_dungeon_viz;

struct DungeonStrandingExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DungeonStrandingExtension {}

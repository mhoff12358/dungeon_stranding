use godot::prelude::*;

#[macro_use]
pub mod godot_utils;

pub mod app;
pub mod available_interactions_viz;
pub mod fight_viz;
pub mod floor_viz;
pub mod game_state_viz;
pub mod in_dungeon_viz;
pub mod interaction_viz;
pub mod inventory_viz;
pub mod my_gd_ref;
pub mod out_of_dungeon_viz;
pub mod shop_item_viz;
pub mod shop_viz;
pub mod template_spawner;
pub mod tree_utils;

struct DungeonStrandingExtension;

#[gdextension]
unsafe impl ExtensionLibrary for DungeonStrandingExtension {}

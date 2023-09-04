use std::ops::Deref;

use ds_lib::{
    game_state::game_state::GameState,
    party_state::inventory::{Inventory, ItemInfo},
};
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};
use owning_ref::OwningHandle;

use crate::{
    game_state_viz::{borrow_game_state, GameStateViz},
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct FloorFiz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl FloorFiz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<GameStateViz> {
        self.in_dungeon.as_ref().unwrap().share()
    }
}

#[godot_api]
impl ControlVirtual for FloorFiz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
    }
}

use std::borrow::Borrow;

use ds_lib::{game_state::game_state::GameState, party_state::inventory::ItemInfo};
use godot::{
    engine::{item_list, Control, ControlVirtual},
    prelude::*,
};

use crate::game_state::{borrow_game_state, GameStateViz};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct OutOfDungeonViz {
    game_state: Option<Gd<GameStateViz>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl OutOfDungeonViz {
    #[func]
    pub fn is_out_of_dungeon(&self) -> bool {
        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        return game_state.is_out_of_dungeon();
    }

    #[func]
    pub fn shop_list(&self) -> GodotString {
        let mut results = Vec::new();

        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        if game_state.is_out_of_dungeon() {
            let shop = &game_state.unwrap_out_of_dungeon().shop;
            let num_items = shop.display_order().len();
            for item_index in 0..num_items {
                let item_for_sale = shop.get_item(item_index).unwrap();
                results.push(format!(
                    "{} | {}",
                    item_for_sale.price,
                    item_for_sale.item.name()
                ));
            }
        }

        return results.join("\n").into();
    }
}

#[godot_api]
impl ControlVirtual for OutOfDungeonViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(self.base.get_parent().unwrap().cast());
    }
}

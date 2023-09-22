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
pub struct InventoryViz {
    game_state: Option<Gd<GameStateViz>>,

    #[base]
    base: Base<Control>,
}

impl InventoryViz {
    fn inventory<'a>(&'a self) -> impl Deref<Target = Inventory> + 'a {
        OwningHandle::new_with_fn(
            borrow_game_state(&self.game_state.as_ref().unwrap()),
            |it: *const GameState| {
                let it = unsafe { &*it };
                if it.is_in_dungeon() {
                    &it.unwrap_in_dungeon().party_stats.inventory
                } else {
                    &it.unwrap_out_of_dungeon().purchased_inventory
                }
            },
        )
    }
}

#[godot_api]
impl InventoryViz {
    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().clone()
    }

    #[func]
    pub fn money(&self) -> i32 {
        self.inventory().get_cash()
    }

    #[func]
    pub fn food(&self) -> i32 {
        self.inventory().get_food()
    }

    #[func]
    pub fn weight(&self) -> i32 {
        self.inventory().total_weight()
    }

    #[func]
    pub fn gear(&self) -> Array<VariantArray> {
        let mut result = Array::new();

        let inventory = self.inventory();
        let gear_counts = inventory.gear_counts();
        for (gear_type, count) in gear_counts.iter() {
            let mut gear_array = Array::new();
            gear_array.push(Into::<GodotString>::into(gear_type.name()).to_variant());
            gear_array.push(Variant::from(*count as i32));
            result.push(gear_array);
        }

        return result;
    }

    #[func]
    pub fn combat_equipment(&self) -> Array<VariantArray> {
        let mut result = Array::new();

        let inventory = self.inventory();
        for id in inventory.combat_equipment_ids().iter() {
            let item = inventory.get_item(id).unwrap();
            let mut item_array = Array::new();
            item_array.push(Into::<GodotString>::into(item.name()).to_variant());
            item_array.push(Into::<GodotString>::into(item.description()).to_variant());
            result.push(item_array);
        }

        return result;
    }
}

#[godot_api]
impl ControlVirtual for InventoryViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(walk_parents_for(&self.base));
    }
}

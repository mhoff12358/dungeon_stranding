use std::ops::Deref;

use ds_lib::{
    game_state::game_state::GameState,
    party_state::inventory::{Inventory, InventoryItem, ItemInfo},
    shop::{shop::Shop, shop_interface::ShopInterface},
};
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};
use owning_ref::OwningHandle;

use crate::{
    game_state_viz::{borrow_game_state, GameStateViz},
    out_of_dungeon_viz::OutOfDungeonViz,
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ShopViz {
    game_state: Option<Gd<GameStateViz>>,
    out_of_dungeon: Option<Gd<OutOfDungeonViz>>,

    #[base]
    base: Base<Control>,
}

impl ShopViz {
    fn items_for_sale_text(shop: &Shop, selected_index: usize) -> String {
        let mut text = String::new();

        let num_items_for_sale = shop.display_order().len();
        let selection_identifier = |selected| if selected { "->" } else { "  " };
        for i in 0..num_items_for_sale {
            let item_for_sale = shop.get_item(i).unwrap();
            text = text
                + format!(
                    "{}| {} | {}\n",
                    selection_identifier(selected_index == i),
                    item_for_sale.price,
                    item_for_sale.item.name()
                )
                .as_str();
        }
        text = text
            + format!(
                "{}| Finish buying and enter the dungeon.",
                selection_identifier(selected_index == num_items_for_sale),
            )
            .as_str();
        return text;
    }

    fn selected_item_text(shop: &Shop, selected_index: usize) -> String {
        if let Some(item) = shop.get_item(selected_index) {
            match &item.item {
                InventoryItem::CombatEquipment(equipment) => {
                    return format!(
                        "Name: {}\nEffect: {}",
                        equipment.name(),
                        equipment.description()
                    );
                }
                InventoryItem::Gear(gear) => {
                    return format!("Gear: {}", gear.name());
                }
            }
        } else {
            return String::from("");
        }
    }

    fn shop<'a>(&'a self) -> impl Deref<Target = Shop> + 'a {
        OwningHandle::new_with_fn(
            borrow_game_state(&self.game_state.as_ref().unwrap()),
            |it: *const GameState| {
                let it = unsafe { &*it };
                &it.unwrap_out_of_dungeon().shop
            },
        )
    }

    fn shop_interface<'a>(&'a self) -> impl Deref<Target = ShopInterface> + 'a {
        OwningHandle::new_with_fn(
            borrow_game_state(&self.game_state.as_ref().unwrap()),
            |it: *const GameState| {
                let it = unsafe { &*it };
                &it.unwrap_out_of_dungeon().shop_interface
            },
        )
    }
}

#[godot_api]
impl ShopViz {
    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().share()
    }

    #[func]
    pub fn out_of_dungeon(&self) -> Gd<OutOfDungeonViz> {
        self.out_of_dungeon.as_ref().unwrap().share()
    }

    #[func]
    fn shop_text(&self) -> GodotString {
        Self::items_for_sale_text(&self.shop(), self.shop_interface().selected_index()).into()
    }

    #[func]
    fn selected_text(&self) -> GodotString {
        Self::selected_item_text(&self.shop(), self.shop_interface().selected_index()).into()
    }
}

#[godot_api]
impl ControlVirtual for ShopViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            out_of_dungeon: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(walk_parents_for(&self.base));
        self.out_of_dungeon = Some(walk_parents_for(&self.base));
    }
}

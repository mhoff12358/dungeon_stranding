use std::{cell::RefCell, rc::Rc};

use ds_lib::game_state::items::inventory::Inventory;
use godot::{
    engine::{ColorRect, Control, IControl, Label},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    game_state_viz::{borrow_game_state, GameStateViz},
    inventory_weight_display::update_weight_display,
    template_spawners::inventory_template_spawner::{
        InventoryItemViz, InventorySpawnerType, InventoryTemplateSpawner,
    },
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InventoryViz {
    game_state: Option<Gd<GameStateViz>>,

    gear_spawner: Option<InventoryTemplateSpawner<InventoryItemViz>>,
    #[export]
    gear_template: Option<Gd<InventoryItemViz>>,

    equipment_spawner: Option<InventoryTemplateSpawner<InventoryItemViz>>,
    #[export]
    equipment_template: Option<Gd<InventoryItemViz>>,

    #[export]
    weight_label: Option<Gd<Label>>,

    #[export]
    gold_label: Option<Gd<Label>>,

    #[export]
    food_label: Option<Gd<Label>>,

    weight_bar_filled: Option<Gd<Control>>,

    base: Base<Control>,
}

impl InventoryViz {
    fn inventory(&self) -> Rc<RefCell<Inventory>> {
        let game_state = borrow_game_state(self.game_state.as_ref().unwrap());
        if game_state.is_in_dungeon() {
            game_state.unwrap_in_dungeon().party_stats.inventory.clone()
        } else {
            game_state.unwrap_out_of_dungeon().party_inventory.clone()
        }
    }
}

#[godot_api]
impl InventoryViz {
    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().clone()
    }

    #[func]
    pub fn update(&mut self) {
        let inventory_rc = self.inventory();
        let inventory = inventory_rc.borrow();
        let food = inventory.get_food();
        self.food_label
            .as_mut()
            .unwrap()
            .set_text(format!("{}", food).into());

        let gold = inventory.get_cash();
        self.gold_label
            .as_mut()
            .unwrap()
            .set_text(format!("{}", gold).into());

        update_weight_display(
            &inventory,
            self.weight_label.as_mut().unwrap(),
            self.weight_bar_filled.as_mut().unwrap(),
        );

        self.equipment_spawner.as_mut().unwrap().update(&inventory);
        self.gear_spawner.as_mut().unwrap().update(&inventory);
    }
}

#[godot_api]
impl IControl for InventoryViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            gear_template: None,
            gear_spawner: None,
            equipment_template: None,
            equipment_spawner: None,
            gold_label: None,
            weight_label: None,
            food_label: None,
            weight_bar_filled: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        let mut game_state: Gd<GameStateViz> = walk_parents_for(&self.to_gd());
        game_state.connect(
            GameStateViz::UPDATED_STATE_SIGNAL.into(),
            self.base().callable("update"),
        );
        self.game_state = Some(game_state);
    }

    fn ready(&mut self) {
        self.gear_spawner = Some(InventoryTemplateSpawner::new(
            self.gear_template.as_ref().unwrap().clone(),
            InventorySpawnerType::Gear,
        ));
        self.equipment_spawner = Some(InventoryTemplateSpawner::new(
            self.equipment_template.as_ref().unwrap().clone(),
            InventorySpawnerType::CombatEquipment,
        ));

        let context = DiContext::get_nearest(self.to_gd().upcast()).unwrap();
        let context = context.bind();
        self.weight_bar_filled = Some(
            context
                .get_registered_node_template::<ColorRect>("weight_bar_filled".into())
                .upcast(),
        );
    }
}

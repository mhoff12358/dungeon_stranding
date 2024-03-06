use std::{cell::RefCell, rc::Rc};

use ds_lib::game_state::items::{inventory::Inventory, unique_id::UniqueItemId};
use godot::{
    engine::{ColorRect, Control, IControl, Label},
    prelude::*,
};
use owning_ref::RefRef;

use crate::{
    di_context::di_context::DiContext,
    game_state_viz::GameStateViz,
    inventory_weight_display::update_weight_display,
    template_spawners::inventory_template_spawner::{
        InventorySpawnerType, InventoryTemplateSpawner,
    },
    tree_utils::walk_parents_for,
};

use super::{
    loot_viz::{LootDirection, LootViz},
    transfer_viz::TransferType,
    transferrable_inventory_item_viz::TransferrableInventoryItemViz,
};

struct Details {
    _other: Gd<TransferrableInventoryViz>,
    inventory: Rc<RefCell<Inventory>>,
    direction: LootDirection,
}

struct RegisteredNodes {
    inventory_name: Gd<Label>,
    money_amount: Gd<Label>,
    food_amount: Gd<Label>,
    weight_amount: Gd<Label>,
    weight_bar_filled: Gd<Control>,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TransferrableInventoryViz {
    loot_viz: Option<Gd<LootViz>>,

    details: Option<Details>,

    inventory_spawner: Option<RefCell<InventoryTemplateSpawner<TransferrableInventoryItemViz>>>,
    registered_nodes: Option<RegisteredNodes>,

    base: Base<Control>,
}

#[godot_api]
impl TransferrableInventoryViz {
    fn reg_mut(&mut self) -> &mut RegisteredNodes {
        self.registered_nodes.as_mut().unwrap()
    }

    #[func(gd_self)]
    pub fn updated(mut this: Gd<Self>) {
        {
            let mut _self = this.bind_mut();
            if let Some(details) = &_self.details {
                let inventory_shared = details.inventory.clone();
                let inventory = inventory_shared.borrow();
                _self
                    .reg_mut()
                    .inventory_name
                    .set_text(inventory.get_display_name().into());
                _self
                    .reg_mut()
                    .money_amount
                    .set_text(format!("{}", inventory.get_cash()).into());
                _self
                    .reg_mut()
                    .food_amount
                    .set_text(format!("{}", inventory.get_food()).into());

                let registered_nodes = _self.reg_mut();
                update_weight_display(
                    &inventory,
                    &mut registered_nodes.weight_amount,
                    &mut registered_nodes.weight_bar_filled,
                );
            }
        }
        let _self = this.bind();
        if _self.details.is_some() {
            let spawner = _self.inventory_spawner.as_ref().unwrap();
            spawner.borrow_mut().update(&this);
        }
    }

    #[func(gd_self)]
    pub fn start_transfer_money(this: Gd<Self>) {
        Self::start_transfer(this, TransferType::Money);
    }

    #[func(gd_self)]
    pub fn start_transfer_food(this: Gd<Self>) {
        Self::start_transfer(this, TransferType::Food);
    }

    pub fn start_transfer(this: Gd<Self>, transfer_type: TransferType) {
        let mut loot_viz;
        let direction;
        {
            let _self = this.bind();
            loot_viz = _self.loot_viz.as_ref().unwrap().clone();
            direction = _self.details.as_ref().unwrap().direction;
        }
        loot_viz
            .bind_mut()
            .start_transfer_amount(transfer_type, direction);
    }

    #[func(gd_self)]
    pub fn transfer_all(this: Gd<Self>) {
        let loot_viz;
        let direction;
        {
            let _self = this.bind();
            loot_viz = _self.loot_viz.as_ref().unwrap().clone();
            direction = _self.details.as_ref().unwrap().direction;
        }
        LootViz::transfer_all(loot_viz, direction);
    }
}

impl TransferrableInventoryViz {
    pub fn init(
        &mut self,
        other: Gd<TransferrableInventoryViz>,
        inventory: Rc<RefCell<Inventory>>,
        direction: LootDirection,
    ) {
        self.details = Some(Details {
            _other: other,
            inventory: inventory.clone(),
            direction,
        });
    }

    pub fn uninit(&mut self) {
        self.details = None;
    }

    pub fn get_inventory(&self) -> RefRef<Inventory> {
        RefRef::new(self.details.as_ref().unwrap().inventory.borrow())
    }

    pub fn get_inventory_rc(&self) -> Rc<RefCell<Inventory>> {
        self.details.as_ref().unwrap().inventory.clone()
    }

    pub fn transfer_item(this: Gd<Self>, item: UniqueItemId) {
        let loot_viz = this.bind().loot_viz.as_ref().unwrap().clone();
        let direction = this.bind().details.as_ref().unwrap().direction;
        LootViz::transfer_item(loot_viz, item, direction);
    }
}

#[godot_api]
impl IControl for TransferrableInventoryViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            loot_viz: None,
            details: None,
            inventory_spawner: None,
            registered_nodes: None,
            base,
        }
    }

    fn ready(&mut self) {
        let di_context = DiContext::get_nearest(self.base().clone().upcast()).unwrap();
        let di_context = di_context.bind();
        let item_template =
            di_context.get_registered_node_template::<TransferrableInventoryItemViz>("".into());
        self.inventory_spawner = Some(RefCell::new(InventoryTemplateSpawner::new(
            item_template,
            InventorySpawnerType::All,
        )));
        self.loot_viz = Some(walk_parents_for(&self.to_gd()));
        let mut game_state: Gd<GameStateViz> = walk_parents_for(&self.to_gd());
        game_state.connect(
            GameStateViz::UPDATED_STATE_SIGNAL.into(),
            self.base().callable("updated"),
        );
        self.registered_nodes = Some(RegisteredNodes {
            inventory_name: di_context.get_registered_node_template("inventory_name".into()),
            money_amount: di_context.get_registered_node_template("money".into()),
            food_amount: di_context.get_registered_node_template("food".into()),
            weight_amount: di_context.get_registered_node_template("weight".into()),
            weight_bar_filled: di_context
                .get_registered_node_template::<ColorRect>("weight_bar_filled".into())
                .upcast(),
        });
    }
}

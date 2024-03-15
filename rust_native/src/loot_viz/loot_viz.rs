use std::{cell::RefCell, rc::Rc};

use ds_lib::game_state::{
    game_state::{GameState, InDungeon, InDungeonEvent, OngoingInteraction},
    inputs::loot_input::LootInput,
    items::{
        inventory::Inventory,
        inventory_transfer::{InventoryTransfer, OngoingInventoryTransfer, TransferIdentifier},
    },
};
use godot::{
    engine::{Control, IControl},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    game_state_viz::{borrow_game_state, GameStateViz},
    tree_utils::walk_parents_for,
};

use super::{
    transfer_viz::{TransferType, TransferViz},
    transferrable_inventory_viz::TransferrableInventoryViz,
};

pub struct DirectedInventories {
    pub from: Rc<RefCell<Inventory>>,
    pub to: Rc<RefCell<Inventory>>,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct LootViz {
    game_state: Option<Gd<GameStateViz>>,

    ongoing_transfer: Option<Rc<RefCell<OngoingInventoryTransfer>>>,

    inventory_display_from: Option<Gd<TransferrableInventoryViz>>,
    inventory_display_to: Option<Gd<TransferrableInventoryViz>>,
    transfer_viz: Option<Gd<TransferViz>>,
    failed_to_finish_popup: Option<Gd<Control>>,

    base: Base<Control>,
}

#[godot_api]
impl LootViz {
    fn game_in_looting_interaction(&self, game_state: &GameState) -> bool {
        return match game_state {
            GameState::InDungeon(InDungeon {
                ongoing_event: Some(InDungeonEvent::Interaction(OngoingInteraction::Loot { .. })),
                ..
            }) => true,
            _ => false,
        };
    }

    #[func(gd_self)]
    pub fn pre_updated(mut this: Gd<Self>) {
        this.set_visible(false);

        let mut _self = this.bind_mut();

        let should_clear;
        {
            let game_state = borrow_game_state(_self.game_state.as_ref().unwrap());
            should_clear = _self.game_in_looting_interaction(&game_state);
        }
        if should_clear {
            _self.clear_transfer_ui();
        }
    }

    #[func(gd_self)]
    pub fn finish(mut this: Gd<Self>) {
        let game_state;
        {
            let mut _self = this.bind_mut();
            game_state = _self.game_state.as_ref().unwrap().clone();
        }
        GameStateViz::accept_input(game_state, &LootInput::finish());
        let mut _self = this.bind_mut();
        if _self.game_in_looting_interaction(&borrow_game_state(_self.game_state.as_ref().unwrap()))
        {
            _self
                .failed_to_finish_popup
                .as_mut()
                .unwrap()
                .set_visible(true);
        }
    }
}

impl LootViz {
    pub fn clear_transfer_ui(&mut self) {
        self.ongoing_transfer = None;
    }

    pub fn set_transfer_ui(&mut self, ongoing_transfer: Rc<RefCell<OngoingInventoryTransfer>>) {
        if let Some(previous_transfer) = self.ongoing_transfer.as_ref() {
            if Rc::ptr_eq(previous_transfer, &ongoing_transfer) {
                return;
            }
        }
        match (
            &mut self.inventory_display_from,
            &mut self.inventory_display_to,
        ) {
            (Some(from), Some(to)) => {
                from.bind_mut().init(
                    to.clone(),
                    ongoing_transfer.borrow().from.clone(),
                    TransferIdentifier::From,
                );
                to.bind_mut().init(
                    from.clone(),
                    ongoing_transfer.borrow().to.clone(),
                    TransferIdentifier::To,
                );
                self.ongoing_transfer = Some(ongoing_transfer);
            }
            _ => {
                panic!("Setting transfer UI missing either a from or to display.");
            }
        }
    }

    pub fn updated(&mut self) {
        self.base_mut().set_visible(true);

        let game_state_transfer;
        {
            let game_state = borrow_game_state(self.game_state.as_ref().unwrap());
            if let OngoingInteraction::Loot { transfer, .. } = game_state
                .unwrap_in_dungeon()
                .ongoing_event
                .as_ref()
                .unwrap()
                .unwrap_interaction()
            {
                game_state_transfer = transfer.clone();
            } else {
                panic!("Updating LootViz without an ongoing loot interaction");
            }
        }
        if self.ongoing_transfer.is_none()
            || (self.ongoing_transfer.as_ref().unwrap() != &game_state_transfer)
        {
            ds_lib::log!("Starting ongoing transfer visualization");
            self.set_transfer_ui(game_state_transfer);
        }
        TransferrableInventoryViz::updated(self.inventory_display_from.clone().unwrap());
        TransferrableInventoryViz::updated(self.inventory_display_to.clone().unwrap());
    }

    fn directed_inventories(&self, direction: TransferIdentifier) -> DirectedInventories {
        match direction {
            TransferIdentifier::From => DirectedInventories {
                from: self
                    .inventory_display_from
                    .as_ref()
                    .unwrap()
                    .bind()
                    .get_inventory_rc(),
                to: self
                    .inventory_display_to
                    .as_ref()
                    .unwrap()
                    .bind()
                    .get_inventory_rc(),
            },
            TransferIdentifier::To => DirectedInventories {
                from: self
                    .inventory_display_to
                    .as_ref()
                    .unwrap()
                    .bind()
                    .get_inventory_rc(),
                to: self
                    .inventory_display_from
                    .as_ref()
                    .unwrap()
                    .bind()
                    .get_inventory_rc(),
            },
        }
    }

    pub fn transfer(mut this: Gd<Self>, details: InventoryTransfer) {
        let game_state;
        {
            let mut _self = this.bind_mut();
            game_state = _self.game_state.as_ref().unwrap().clone();
        }
        GameStateViz::accept_input(game_state, &LootInput::transfer(details));
    }

    /*pub fn transfer_all(this: Gd<Self>, direction: TransferIdentifier) {
        let game_state: Gd<GameStateViz>;
        {
            let _self = this.bind();
            game_state = _self.game_state.as_ref().unwrap().clone();

            let inventories = _self.directed_inventories(direction);
            inventories
                .to
                .borrow_mut()
                .empty_other(&mut inventories.from.borrow_mut());
        }
        GameStateViz::handle_game_update(game_state);
    }*/

    pub fn start_transfer_amount(
        &mut self,
        transfer_type: TransferType,
        direction: TransferIdentifier,
    ) {
        let inventories = self.directed_inventories(direction);
        TransferViz::init(
            self.transfer_viz.as_mut().unwrap().clone(),
            transfer_type,
            direction,
            inventories,
        );
    }
}

#[godot_api]
impl IControl for LootViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            ongoing_transfer: None,
            inventory_display_from: None,
            inventory_display_to: None,
            transfer_viz: None,
            failed_to_finish_popup: None,
            base,
        }
    }

    fn ready(&mut self) {
        let mut game_state: Gd<GameStateViz> = walk_parents_for(&self.to_gd());
        self.game_state = Some(game_state.clone());
        game_state.connect(
            GameStateViz::PRE_UPDATED_STATE_SIGNAL.into(),
            self.base().callable("pre_updated"),
        );

        let di_context = DiContext::get_nearest(self.base().clone().upcast()).unwrap();
        let di_context = di_context.bind();

        self.inventory_display_to =
            Some(di_context.get_registered_node_template::<TransferrableInventoryViz>("to".into()));
        self.inventory_display_from = Some(
            di_context.get_registered_node_template::<TransferrableInventoryViz>("from".into()),
        );
        self.transfer_viz = Some(di_context.get_registered_node_template::<TransferViz>("".into()));
        self.failed_to_finish_popup = Some(
            di_context.get_registered_node_template::<Control>("failed_to_finish_popup".into()),
        );
    }
}

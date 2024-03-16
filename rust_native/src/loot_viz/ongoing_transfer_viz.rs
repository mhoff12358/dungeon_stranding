use std::{cell::RefCell, rc::Rc};

use ds_lib::game_state::{
    inputs::game_state_input::GameStateInput,
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
    di_context::di_context::DiContext, game_state_viz::GameStateViz, tree_utils::walk_parents_for,
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
pub struct OngoingTransferViz {
    game_state: Option<Gd<GameStateViz>>,

    ongoing_transfer: Option<Rc<RefCell<OngoingInventoryTransfer>>>,

    inventory_display_from: Option<Gd<TransferrableInventoryViz>>,
    inventory_display_to: Option<Gd<TransferrableInventoryViz>>,
    transfer_viz: Option<Gd<TransferViz>>,
    failed_to_finish_popup: Option<Gd<Control>>,

    inputs: Option<(
        Box<dyn Fn(InventoryTransfer) -> GameStateInput>,
        Box<dyn Fn() -> GameStateInput>,
    )>,

    base: Base<Control>,
}

#[godot_api]
impl OngoingTransferViz {
    pub fn clear_transfer_ui(&mut self) {
        self.base_mut().set_visible(false);

        self.ongoing_transfer = None;
    }

    pub fn set_transfer_ui(&mut self, ongoing_transfer: &Rc<RefCell<OngoingInventoryTransfer>>) {
        self.base_mut().set_visible(true);

        if let Some(previous_transfer) = self.ongoing_transfer.as_ref() {
            if Rc::ptr_eq(previous_transfer, ongoing_transfer) {
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
                self.ongoing_transfer = Some(ongoing_transfer.clone());
            }
            _ => {
                panic!("Setting transfer UI missing either a from or to display.");
            }
        }
    }

    pub fn updated(
        mut this: Gd<OngoingTransferViz>,
        ongoing_transfer: &Rc<RefCell<OngoingInventoryTransfer>>,
    ) {
        this.set_visible(true);

        {
            let mut _self = this.bind_mut();
            if _self.ongoing_transfer.is_none()
                || (_self.ongoing_transfer.as_ref().unwrap() != ongoing_transfer)
            {
                ds_lib::log!("Starting ongoing transfer visualization");
                _self.set_transfer_ui(ongoing_transfer);
            }
            TransferrableInventoryViz::updated(_self.inventory_display_from.clone().unwrap());
            TransferrableInventoryViz::updated(_self.inventory_display_to.clone().unwrap());
        }
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

    pub fn set_inputs(
        &mut self,
        transfer_input: Box<dyn Fn(InventoryTransfer) -> GameStateInput>,
        finish_input: Box<dyn Fn() -> GameStateInput>,
    ) {
        self.inputs = Some((transfer_input, finish_input));
    }

    pub fn transfer(mut this: Gd<Self>, details: InventoryTransfer) {
        let game_state;
        {
            let mut _self = this.bind_mut();
            game_state = _self.game_state.as_ref().unwrap().clone();
        }
        let input = this.bind_mut().inputs.as_ref().unwrap().0(details);
        GameStateViz::accept_input(game_state, &input);
    }

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

    #[func(gd_self)]
    pub fn finish(mut this: Gd<Self>) {
        let game_state;
        {
            let mut _self = this.bind_mut();
            game_state = _self.game_state.as_ref().unwrap().clone();
        }
        let input = this.bind_mut().inputs.as_ref().unwrap().1();
        GameStateViz::accept_input(game_state, &input);
    }
}

#[godot_api]
impl IControl for OngoingTransferViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            ongoing_transfer: None,
            inventory_display_from: None,
            inventory_display_to: None,
            transfer_viz: None,
            inputs: None,
            failed_to_finish_popup: None,
            base,
        }
    }

    fn ready(&mut self) {
        let game_state: Gd<GameStateViz> = walk_parents_for(&self.to_gd());
        self.game_state = Some(game_state.clone());

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

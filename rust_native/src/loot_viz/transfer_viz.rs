use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};

use crate::{game_state_viz::GameStateViz, tree_utils::walk_parents_for};

use super::loot_viz::{LootDirection, LootViz};

#[derive(Debug, Clone, Copy)]
pub enum TransferType {
    Money,
    Food,
}

#[derive(Debug, Clone, Copy)]
pub struct TransferDetails {
    pub direction: LootDirection,
    pub transfer_type: TransferType,
    pub amount: i32,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TransferViz {
    loot_viz: Option<Gd<LootViz>>,

    details: TransferDetails,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl TransferViz {
    pub fn init(&mut self, transfer_type: TransferType, direction: LootDirection) {
        self.details = TransferDetails {
            direction,
            transfer_type,
            amount: 0,
        };
        self.base.set_visible(true);
    }

    #[func]
    pub fn set_amount(&mut self, amount: i32) {
        self.details.amount = amount;
    }

    #[func(gd_self)]
    pub fn apply(mut this: Gd<Self>) {
        let details;
        let loot_viz;
        {
            let mut _self = this.bind_mut();
            loot_viz = _self.loot_viz.as_ref().unwrap().clone();
            details = _self.details;
            _self.base.set_visible(false);
        }
        LootViz::transfer_amount(loot_viz, details);
    }
}

#[godot_api]
impl ControlVirtual for TransferViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            details: TransferDetails {
                direction: LootDirection::From,
                transfer_type: TransferType::Money,
                amount: 0,
            },
            base,
            loot_viz: None,
        }
    }

    fn ready(&mut self) {
        self.loot_viz = Some(walk_parents_for(&self.base));
    }
}

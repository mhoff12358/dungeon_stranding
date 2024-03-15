use ds_lib::game_state::items::inventory_transfer::{
    InventoryTransfer, Transfer, TransferIdentifier,
};
use godot::{
    engine::{Control, IControl, Label, Slider},
    prelude::*,
};

use crate::{di_context::di_context::DiContext, tree_utils::walk_parents_for};

use super::loot_viz::{DirectedInventories, LootViz};

#[derive(Debug, Clone, Copy)]
pub enum TransferType {
    Money,
    Food,
}

#[derive(Debug, Clone, Copy)]
pub struct TransferDetails {
    pub transfer_type: TransferType,
    pub source: TransferIdentifier,
    pub amount: i32,
}

struct Components {
    description: Gd<Label>,
    min: Gd<Label>,
    max: Gd<Label>,
    amount: Gd<Label>,

    slider: Gd<Slider>,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TransferViz {
    loot_viz: Option<Gd<LootViz>>,

    details: TransferDetails,
    min: i32,
    max: i32,

    components: Option<Components>,

    base: Base<Control>,
}

#[godot_api]
impl TransferViz {
    pub fn init(
        mut this: Gd<Self>,
        transfer_type: TransferType,
        source: TransferIdentifier,
        directed_inventories: DirectedInventories,
    ) {
        let mut slider;
        let (min, max): (i32, i32);
        {
            let mut _self = this.bind_mut();
            _self.details = TransferDetails {
                transfer_type,
                source,
                amount: 0,
            };

            let description;
            match transfer_type {
                TransferType::Money => {
                    description = "money";
                    _self.min = 0;
                    _self.max = directed_inventories.from.borrow().get_cash();
                }
                TransferType::Food => {
                    description = "food";
                    _self.min = 0;
                    _self.max = directed_inventories.from.borrow().get_food();
                }
            }

            let description = format!(
                "Transferring {} from {} to {}",
                description,
                directed_inventories.from.borrow().get_display_name(),
                directed_inventories.to.borrow().get_display_name()
            );

            _self.base_mut().set_visible(true);

            (min, max) = (_self.min, _self.max);
            _self.set_amount(min as f32);
            let components = _self.components.as_mut().unwrap();
            components.description.set_text(description.into());
            components.min.set_text(format!("{}", min).into());
            components.max.set_text(format!("{}", max).into());

            slider = components.slider.clone();
        }

        slider.set_min(min as f64);
        slider.set_max(max as f64);
        slider.set_value(max as f64);
    }

    #[func]
    pub fn set_amount(&mut self, amount: f32) {
        let amount = (amount as i32).min(self.max).max(self.min);

        self.details.amount = amount;
        self.components
            .as_mut()
            .unwrap()
            .amount
            .set_text(format!("{}", amount).into());
    }

    #[func(gd_self)]
    pub fn apply(mut this: Gd<Self>) {
        let details;
        let loot_viz;
        {
            let mut _self = this.bind_mut();
            loot_viz = _self.loot_viz.as_ref().unwrap().clone();
            details = _self.details;
            _self.base_mut().set_visible(false);
        }
        LootViz::transfer(
            loot_viz,
            InventoryTransfer {
                source_inventory: details.source,
                transfer: match details.transfer_type {
                    TransferType::Food => Transfer::Food {
                        amount: details.amount,
                    },
                    TransferType::Money => Transfer::Gold {
                        amount: details.amount,
                    },
                },
            },
        );
    }
}

#[godot_api]
impl IControl for TransferViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            details: TransferDetails {
                source: TransferIdentifier::From,
                transfer_type: TransferType::Money,
                amount: 0,
            },
            min: 0,
            max: 0,
            components: None,
            loot_viz: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.loot_viz = Some(walk_parents_for(&self.to_gd()));
        self.base_mut().set_visible(false);

        let di_context = DiContext::get_nearest_bound(self.base().clone());
        self.components = Some(Components {
            description: di_context.get_registered_node_template("description".into()),
            min: di_context.get_registered_node_template("min".into()),
            max: di_context.get_registered_node_template("max".into()),
            amount: di_context.get_registered_node_template("amount".into()),
            slider: di_context.get_registered_node_template("slider".into()),
        })
    }
}

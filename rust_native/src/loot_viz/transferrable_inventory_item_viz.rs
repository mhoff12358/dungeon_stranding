use std::ops::Deref;

use ds_lib::game_state::items::{inventory::Inventory, items::ItemInfo, unique_id::UniqueItemId};
use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};
use owning_ref::{OwningHandle, StableAddress};

use crate::{
    my_gd_ref::MyGdRef,
    template_spawners::{
        inventory_template_spawner::{ContextProvidesInventory, ProvidedInventory},
        template_spawner::TemplateControl,
    },
};

use super::transferrable_inventory_viz::TransferrableInventoryViz;

struct TransferInfo {
    transferer: Gd<TransferrableInventoryViz>,
    item: UniqueItemId,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct TransferrableInventoryItemViz {
    #[export]
    label: Option<Gd<Label>>,

    info: Option<TransferInfo>,

    base: Base<Control>,
}

#[godot_api]
impl TransferrableInventoryItemViz {
    #[func]
    pub fn transfer(&self) {
        let info = self.info.as_ref().unwrap();
        TransferrableInventoryViz::transfer_item(info.transferer.clone(), info.item);
    }
}

impl ContextProvidesInventory for Gd<TransferrableInventoryViz> {
    fn inventory<'a>(&'a self) -> ProvidedInventory<'a> {
        fn internal_borrow<'b>(
            it: *const TransferrableInventoryViz,
        ) -> impl Deref<Target = Inventory> + StableAddress + 'b {
            let it = unsafe { &*it };
            it.get_inventory()
        }
        let handle = OwningHandle::new_with_fn(MyGdRef::new(self.bind()), &internal_borrow);
        ProvidedInventory::Box(Box::new(handle))
    }
}

impl TemplateControl for TransferrableInventoryItemViz {
    type Value = UniqueItemId;
    type Context = Gd<TransferrableInventoryViz>;

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context) {
        self.info = Some(TransferInfo {
            transferer: context.clone(),
            item: *value,
        });
        let transferrable_inventory = context.bind();
        let inventory = transferrable_inventory.get_inventory();
        let item = inventory.get_item(value).unwrap();
        if let Some(label) = self.label.as_mut() {
            label.set_text(format!("{}: {}", item.name(), item.description()).into());
        }
    }
}

#[godot_api]
impl IControl for TransferrableInventoryItemViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            base,
            label: None,
            info: None,
        }
    }
}

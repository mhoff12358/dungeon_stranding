use std::{cell::RefCell, ops::Deref, rc::Rc};

use ds_lib::game_state::inventory::{Inventory, ItemInfo, UniqueItemId};
use godot::{
    engine::{Control, ControlVirtual, Label},
    prelude::*,
};

use super::template_spawner::{Template, TemplateControl, TemplateSpawner};

pub enum InventorySpawnerType {
    Gear,
    CombatEquipment,
    All,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InventoryItemViz {
    #[export]
    label: Option<Gd<Label>>,

    base: Base<Control>,
}

#[godot_api]
impl InventoryItemViz {}

impl TemplateControl for InventoryItemViz {
    type Value = UniqueItemId;
    type Context = Inventory;

    fn control(&self) -> &Self::Base {
        &self.base
    }

    fn control_mut(&mut self) -> &mut Self::Base {
        &mut self.base
    }

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context) {
        let item = context.get_item(value).unwrap();
        if let Some(label) = self.label.as_mut() {
            label.set_text(format!("{}: {}", item.name(), item.description()).into());
        }
    }
}

#[godot_api]
impl ControlVirtual for InventoryItemViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self { label: None, base }
    }
}

pub enum ProvidedInventory<'a> {
    Ref(&'a Inventory),
    Box(Box<dyn Deref<Target = Inventory> + 'a>),
}

impl<'a> Deref for ProvidedInventory<'a> {
    type Target = Inventory;

    fn deref(&self) -> &Self::Target {
        match self {
            ProvidedInventory::Ref(inventory) => inventory,
            ProvidedInventory::Box(provider) => &provider,
        }
    }
}

pub trait ContextProvidesInventory {
    fn inventory(&self) -> ProvidedInventory;
}

impl ContextProvidesInventory for Inventory {
    fn inventory(&self) -> ProvidedInventory {
        ProvidedInventory::Ref(&self)
    }
}

pub struct InventoryTemplateSpawner<TemplateType: Template<Value = UniqueItemId>>
where
    TemplateType::Context: ContextProvidesInventory,
{
    spawner: TemplateSpawner<UniqueItemId, UniqueItemId, TemplateType::Context, TemplateType>,
    spawner_type: InventorySpawnerType,
    //inventory: Option<Rc<RefCell<Inventory>>>,
}

impl<TemplateType: Template<Value = UniqueItemId>> InventoryTemplateSpawner<TemplateType>
where
    TemplateType::Context: ContextProvidesInventory,
{
    pub fn new(template: Gd<TemplateType>, spawner_type: InventorySpawnerType) -> Self {
        Self {
            spawner: TemplateSpawner::new(template),
            spawner_type,
            //inventory: None,
        }
    }

    pub fn update(&mut self, context: &TemplateType::Context) {
        match self.spawner_type {
            InventorySpawnerType::Gear => self
                .spawner
                .update_ref(context.inventory().gear_ids().iter(), context),
            InventorySpawnerType::CombatEquipment => self
                .spawner
                .update_ref(context.inventory().combat_equipment_ids().iter(), context),
            InventorySpawnerType::All => self
                .spawner
                .update_ref(context.inventory().all_items(), context),
        };
    }
}

use std::{marker::PhantomData, ops::Deref};

use ds_lib::game_state::inventory::{Inventory, ItemInfo, UniqueItemId};
use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};

use super::{
    template_spawner::{Template, TemplateControl, TemplateGenerics, TemplateSpawner},
    update_behavior::UpdateSpawnedTemplate,
};

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

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context) {
        let item = context.get_item(value).unwrap();
        if let Some(label) = self.label.as_mut() {
            label.set_text(format!("{}: {}", item.name(), item.description()).into());
        }
    }
}

#[godot_api]
impl IControl for InventoryItemViz {
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

struct InventoryGenerics<TemplateType: Template<Value = UniqueItemId>> {
    _data: PhantomData<TemplateType>,
}

impl<TemplateType: Template<Value = UniqueItemId>> TemplateGenerics
    for InventoryGenerics<TemplateType>
{
    type Key = UniqueItemId;
    type Value = UniqueItemId;
    type Context = TemplateType::Context;
    type TemplateType = TemplateType;
}

pub struct InventoryTemplateSpawner<TemplateType: Template<Value = UniqueItemId>>
where
    TemplateType::Context: ContextProvidesInventory,
{
    spawner: TemplateSpawner<
        InventoryGenerics<TemplateType>,
        UpdateSpawnedTemplate<InventoryGenerics<TemplateType>>,
    >,
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

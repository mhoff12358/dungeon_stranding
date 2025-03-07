use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
};

use godot::{
    engine::{global::Side, Control},
    obj::{bounds::DeclUser, WithBaseField},
    prelude::*,
};

use crate::packing::pack_node;

use super::update_behavior::TemplateSpawnerUpdateBehavior;

pub trait Template: GodotClass<Declarer = DeclUser> + Inherits<Node> + Sized {
    type Value;
    type Context;

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context);
    fn update_template(
        &mut self,
        value: &Self::Value,
        context: &Self::Context,
        previous: &Option<Gd<Self>>,
    );
}

pub trait TemplateControl:
    GodotClass<Declarer = DeclUser, Base = Control>
    + Inherits<Node>
    + Inherits<Control>
    + WithBaseField
    + Sized
{
    type Value;
    type Context;

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context);
}

impl<T> Template for T
where
    T: TemplateControl,
{
    type Value = <T as TemplateControl>::Value;
    type Context = <T as TemplateControl>::Context;

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context) {
        <T as TemplateControl>::instantiate_template(self, value, context);
    }

    fn update_template(
        &mut self,
        _value: &Self::Value,
        _context: &Self::Context,
        previous: &Option<Gd<Self>>,
    ) {
        let top;
        if let Some(previous) = previous {
            let previous = previous.bind();
            let previous_control = previous.base();
            top = previous_control.get_anchor(Side::BOTTOM);
        } else {
            top = 0.0;
        }

        let mut control = self.base_mut();
        let size = control.get_anchor(Side::BOTTOM) - control.get_anchor(Side::TOP);

        control
            .set_anchor_ex(Side::TOP, top)
            .keep_offset(true)
            .done();
        control
            .set_anchor_ex(Side::BOTTOM, top + size)
            .keep_offset(true)
            .done();
    }
}

pub trait TemplateGenerics {
    type Key: Hash + Eq + PartialEq + Clone;
    type Value;
    type Context;
    type TemplateType: GodotClass + Inherits<Node>;
}

pub struct TemplateSpawner<
    Generics: TemplateGenerics,
    UpdateBehavior: TemplateSpawnerUpdateBehavior<Generics = Generics>,
> {
    parent: Gd<Node>,
    template: Gd<PackedScene>,

    instantiated_templates: HashMap<Generics::Key, Gd<Generics::TemplateType>>,
    phantom: PhantomData<(Generics, UpdateBehavior)>,
}

impl<
        Generics: TemplateGenerics,
        UpdateBehavior: TemplateSpawnerUpdateBehavior<Generics = Generics>,
    > TemplateSpawner<Generics, UpdateBehavior>
{
    pub fn new(template: Gd<Generics::TemplateType>) -> Self {
        let template = template.upcast();
        let mut parent = template.get_parent().unwrap();
        parent.remove_child(template.clone());

        let template_scene = pack_node(template);
        Self {
            parent: parent,
            template: template_scene,
            instantiated_templates: Default::default(),
            phantom: Default::default(),
        }
    }

    fn instantiate_template(
        parent: &mut Gd<Node>,
        template: &Gd<PackedScene>,
        context: &Generics::Context,
        value: &Generics::Value,
    ) -> Gd<Generics::TemplateType> {
        let new_node: Gd<Generics::TemplateType> = template.instantiate().unwrap().cast();
        parent.add_child(new_node.clone().upcast());
        UpdateBehavior::initialize(new_node.clone(), value, context);
        return new_node;
    }

    fn place_instantiated_template_after(
        instantiated_template: &mut Gd<Generics::TemplateType>,
        context: &Generics::Context,
        value: &Generics::Value,
        previous: &Option<Gd<Generics::TemplateType>>,
    ) {
        UpdateBehavior::update_template(instantiated_template.clone(), value, context, previous);
    }

    pub fn update_with_getter<'a, GetKey>(
        &'a mut self,
        values: impl Iterator<Item = impl Deref<Target = Generics::Value> + 'a> + 'a,
        get_key: GetKey,
        context: &Generics::Context,
    ) where
        GetKey: Fn(&Generics::Value) -> Generics::Key,
    {
        let mut unused_keys: HashSet<Generics::Key> = self
            .instantiated_templates
            .keys()
            .map(|key| key.clone())
            .collect();

        let mut previous_node = None;

        let parent = &mut self.parent;
        let template = &self.template;

        for value in values {
            let key = (get_key)(&value);
            let mut instantiated_template = self
                .instantiated_templates
                .entry(key.clone())
                .or_insert_with(|| Self::instantiate_template(parent, template, context, &value))
                .clone();
            unused_keys.remove(&key);

            Self::place_instantiated_template_after(
                &mut instantiated_template,
                context,
                &value,
                &previous_node,
            );

            previous_node = Some(instantiated_template);
        }

        for key in unused_keys.iter() {
            self.instantiated_templates
                .remove(key)
                .unwrap()
                .upcast::<Node>()
                .queue_free();
        }
    }
}

struct RefWrapper<T>(T);

impl<T> Deref for RefWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<
        'a,
        Key: Hash + Eq + PartialEq + Copy + 'a,
        Generics: TemplateGenerics<Key = Key, Value = Key>,
        UpdateBehavior: TemplateSpawnerUpdateBehavior<Generics = Generics>,
    > TemplateSpawner<Generics, UpdateBehavior>
{
    pub fn update(
        &'a mut self,
        values: impl Iterator<Item = Generics::Value>,
        context: &Generics::Context,
    ) {
        self.update_with_getter(values.map(|x| RefWrapper(x)), |x| *x, context);
    }

    pub fn update_ref(
        &'a mut self,
        values: impl Iterator<Item = &'a Generics::Value>,
        context: &Generics::Context,
    ) {
        self.update_with_getter(values, |x| *x, context);
    }
}

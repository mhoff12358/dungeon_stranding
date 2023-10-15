use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
};

use ds_lib::log;
use godot::{
    engine::{global::Side, Control},
    obj::dom::UserDomain,
    prelude::*,
};

use crate::packing::pack_node;

pub trait Template: GodotClass<Declarer = UserDomain> + Inherits<Node> + Sized {
    type Value;
    type Context;

    fn instantiate_template(&mut self, value: &Self::Value, context: &Self::Context);
    fn place_after(&mut self, previous: &Option<Gd<Self>>);
}

pub trait TemplateControl:
    GodotClass<Declarer = UserDomain, Base = Control> + Inherits<Node> + Inherits<Control> + Sized
{
    type Value;
    type Context;

    fn control(&self) -> &Self::Base;
    fn control_mut(&mut self) -> &mut Self::Base;
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

    fn place_after(&mut self, previous: &Option<Gd<Self>>) {
        let top;
        if let Some(previous) = previous {
            let previous = previous.bind();
            let previous_control = previous.control();
            top = previous_control.get_anchor(Side::SIDE_BOTTOM);
        } else {
            top = 0.0;
        }

        let size = self.control().get_anchor(Side::SIDE_BOTTOM)
            - self.control().get_anchor(Side::SIDE_TOP);

        self.control_mut()
            .set_anchor_ex(Side::SIDE_TOP, top)
            .keep_offset(true)
            .done();
        self.control_mut()
            .set_anchor_ex(Side::SIDE_BOTTOM, top + size)
            .keep_offset(true)
            .done();
    }
}

pub struct TemplateSpawner<Key, Value, Context, TemplateType: GodotClass + Inherits<Node>>
where
    Key: Hash + Eq + PartialEq + Copy,
{
    parent: Gd<Node>,
    template: Gd<PackedScene>,

    instantiated_templates: HashMap<Key, Gd<TemplateType>>,
    phantom: PhantomData<(Value, Context)>,
}

pub trait TemplateSpawnerDefaultImplTrait {
    type Value;
    type Context;
    type TemplateType: GodotClass + Inherits<Node>;

    fn initialize(template: Gd<Self::TemplateType>, value: &Self::Value, context: &Self::Context);
    fn place_after(template: Gd<Self::TemplateType>, previous: &Option<Gd<Self::TemplateType>>);
}

impl<Key, Value: ToVariant, Context> TemplateSpawnerDefaultImplTrait
    for TemplateSpawner<Key, Value, Context, Node>
where
    Key: Hash + Eq + PartialEq + Copy,
{
    type Value = Value;
    type Context = ();
    type TemplateType = Node;

    fn initialize(template: Gd<Self::TemplateType>, value: &Self::Value, _context: &Self::Context) {
        template
            .upcast::<Node>()
            .emit_signal("instantiate_template".into(), &[value.to_variant()]);
    }

    fn place_after(template: Gd<Self::TemplateType>, previous: &Option<Gd<Self::TemplateType>>) {
        template.upcast::<Node>().emit_signal(
            "place_after".into(),
            &[if let Some(previous) = previous {
                previous.to_variant()
            } else {
                Variant::nil()
            }],
        );
    }
}

impl<Key, Value, Context, TemplateType: Template<Value = Value, Context = Context>>
    TemplateSpawnerDefaultImplTrait for TemplateSpawner<Key, Value, Context, TemplateType>
where
    Key: Hash + Eq + PartialEq + Copy,
{
    type Value = Value;
    type Context = Context;
    type TemplateType = TemplateType;

    fn initialize(
        mut template: Gd<Self::TemplateType>,
        value: &Self::Value,
        context: &Self::Context,
    ) {
        template.bind_mut().instantiate_template(value, context);
    }

    fn place_after(
        mut template: Gd<Self::TemplateType>,
        previous: &Option<Gd<Self::TemplateType>>,
    ) {
        template.bind_mut().place_after(previous);
    }
}

impl<Key, Value, Context, TemplateType: Inherits<Node>>
    TemplateSpawner<Key, Value, Context, TemplateType>
where
    Key: Hash + Eq + PartialEq + Copy + Debug,
    Self: TemplateSpawnerDefaultImplTrait<
        TemplateType = TemplateType,
        Context = Context,
        Value = Value,
    >,
{
    pub fn new(template: Gd<TemplateType>) -> Self {
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
        context: &Context,
        value: &Value,
    ) -> Gd<TemplateType> {
        let new_node: Gd<TemplateType> = template.instantiate().unwrap().cast();
        parent.add_child(new_node.clone().upcast());
        Self::initialize(new_node.clone(), value, context);
        return new_node;
    }

    fn place_instantiated_template_after(
        instantiated_template: &mut Gd<TemplateType>,
        previous: &Option<Gd<TemplateType>>,
    ) {
        Self::place_after(instantiated_template.clone(), previous);
    }

    pub fn update_with_getter<'a, GetKey>(
        &'a mut self,
        values: impl Iterator<Item = impl Deref<Target = Value> + 'a> + 'a,
        get_key: GetKey,
        context: &Context,
    ) where
        GetKey: Fn(&Value) -> Key,
    {
        let mut unused_keys: HashSet<Key> =
            self.instantiated_templates.keys().map(|key| *key).collect();

        let mut previous_node = None;

        let parent = &mut self.parent;
        let template = &self.template;

        for value in values {
            let key = (get_key)(&value);
            let mut instantiated_template = self
                .instantiated_templates
                .entry(key)
                .or_insert_with(|| Self::instantiate_template(parent, template, context, &value))
                .clone();
            unused_keys.remove(&key);

            Self::place_instantiated_template_after(&mut instantiated_template, &previous_node);

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

impl<Value, Context, TemplateType: Inherits<Node>>
    TemplateSpawner<Value, Value, Context, TemplateType>
where
    Value: Hash + Eq + PartialEq + Copy + Debug,
    Self: TemplateSpawnerDefaultImplTrait<
        TemplateType = TemplateType,
        Value = Value,
        Context = Context,
    >,
{
    pub fn update<'a>(&'a mut self, values: impl Iterator<Item = Value>, context: &Context) {
        self.update_with_getter(values.map(|x| RefWrapper(x)), |x| *x, context);
    }

    pub fn update_ref<'a>(
        &'a mut self,
        values: impl Iterator<Item = &'a Value>,
        context: &Context,
    ) {
        self.update_with_getter(values, |x| *x, context);
    }
}

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use godot::{engine::node::DuplicateFlags, prelude::*};

pub struct TemplateSpawner<Key>
where
    Key: Hash + Eq + PartialEq + Copy,
{
    parent: Gd<Node>,
    template: Gd<Node>,

    instantiated_templates: HashMap<Key, Gd<Node>>,
}

impl<Key> TemplateSpawner<Key>
where
    Key: Hash + Eq + PartialEq + Copy + Debug,
{
    pub fn new(template: Gd<Node>) -> Self {
        let mut parent = template.get_parent().unwrap();
        parent.remove_child(template.clone());
        Self {
            parent: parent,
            template,
            instantiated_templates: Default::default(),
        }
    }

    fn instantiate_template<Value>(
        parent: &mut Gd<Node>,
        template: &Gd<Node>,
        value: &Value,
    ) -> Gd<Node>
    where
        Value: ToVariant,
    {
        let mut new_node = template
            .duplicate_ex()
            .flags(
                DuplicateFlags::DUPLICATE_SCRIPTS.ord()
                    | DuplicateFlags::DUPLICATE_SIGNALS.ord()
                    | DuplicateFlags::DUPLICATE_GROUPS.ord(),
            )
            .done()
            .unwrap();
        parent.add_child(new_node.clone());
        //new_node.emit_signal("instantiate_template".into(), &[value.to_variant()]);
        new_node.call("_on_instantiate_template".into(), &[value.to_variant()]);
        return new_node;
    }

    fn place_instantiated_template_after(
        instantiated_template: &mut Gd<Node>,
        previous: &Option<Gd<Node>>,
    ) {
        instantiated_template.call(
            "_on_place_after".into(),
            &[if let Some(previous) = previous {
                previous.to_variant()
            } else {
                Variant::nil()
            }],
        );
        /*instantiated_template.emit_signal(
            "place_after".into(),
            &[if let Some(previous) = previous {
                previous.to_variant()
            } else {
                Variant::nil()
            }],
        );*/
    }

    pub fn update<'a, Value, GetKey>(
        &mut self,
        values: impl Iterator<Item = Value>,
        get_key: GetKey,
    ) where
        GetKey: Fn(&Value) -> Key,
        Value: 'a + ToVariant + Debug,
    {
        let mut unused_keys: HashSet<Key> =
            self.instantiated_templates.keys().map(|key| *key).collect();

        let mut previous_node = None;

        let parent = &mut self.parent;
        let template = &self.template;

        for value in values {
            let key = (get_key)(&value);
            ds_lib::log!("Found value with key: {:?}, {:?}", &value, &key);
            let mut instantiated_template = self
                .instantiated_templates
                .entry(key)
                .or_insert_with(|| Self::instantiate_template(parent, template, &value))
                .clone();
            unused_keys.remove(&key);

            ds_lib::log!("Placing after");
            Self::place_instantiated_template_after(&mut instantiated_template, &previous_node);

            previous_node = Some(instantiated_template);
        }

        for key in unused_keys.iter() {
            self.instantiated_templates.remove(key);
        }
    }
}

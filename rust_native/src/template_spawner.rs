use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use godot::prelude::*;

pub struct TemplateSpawner<Key>
where
    Key: Hash + Eq + PartialEq + Copy,
{
    parent: Gd<Node>,
    template: Gd<PackedScene>,

    instantiated_templates: HashMap<Key, Gd<Node>>,
}

impl<Key> TemplateSpawner<Key>
where
    Key: Hash + Eq + PartialEq + Copy + Debug,
{
    pub fn new(template: Gd<Node>) -> Self {
        let mut parent = template.get_parent().unwrap();
        parent.remove_child(template.clone());

        let mut template_scene = PackedScene::new();
        let template_children = template.get_children_ex().include_internal(true).done();
        for mut child in template_children.iter_shared() {
            child.set_owner(template.clone());
        }
        template_scene.pack(template);
        Self {
            parent: parent,
            template: template_scene,
            instantiated_templates: Default::default(),
        }
    }

    fn instantiate_template<Value>(
        parent: &mut Gd<Node>,
        template: &Gd<PackedScene>,
        value: &Value,
    ) -> Gd<Node>
    where
        Value: ToVariant,
    {
        let mut new_node = template.instantiate().unwrap();
        parent.add_child(new_node.clone());
        new_node.emit_signal("instantiate_template".into(), &[value.to_variant()]);
        return new_node;
    }

    fn place_instantiated_template_after(
        instantiated_template: &mut Gd<Node>,
        previous: &Option<Gd<Node>>,
    ) {
        instantiated_template.emit_signal(
            "place_after".into(),
            &[if let Some(previous) = previous {
                previous.to_variant()
            } else {
                Variant::nil()
            }],
        );
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
            let mut instantiated_template = self
                .instantiated_templates
                .entry(key)
                .or_insert_with(|| Self::instantiate_template(parent, template, &value))
                .clone();
            unused_keys.remove(&key);

            Self::place_instantiated_template_after(&mut instantiated_template, &previous_node);

            previous_node = Some(instantiated_template);
        }

        for key in unused_keys.iter() {
            self.instantiated_templates
                .remove(key)
                .unwrap()
                .queue_free();
        }
    }
}

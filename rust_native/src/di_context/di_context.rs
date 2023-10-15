use std::{cell::RefCell, collections::HashMap};

use godot::prelude::*;

thread_local! {
    static DI_REGISTRY: RefCell<HashMap<InstanceId, (Gd<Node>, Gd<DiContext>)>> =
        RefCell::new(HashMap::default());
}

fn insert_di_context(parent: Gd<Node>, di_context: Gd<DiContext>) {
    DI_REGISTRY.with(|di_registry| {
        di_registry
            .borrow_mut()
            .insert(parent.instance_id(), (parent, di_context))
    });
}

fn lookup_di_context(parent: &Gd<Node>) -> Option<Gd<DiContext>> {
    let mut result = None;
    DI_REGISTRY.with(|di_registry| {
        if let Some(context) = di_registry.borrow().get(&parent.instance_id()) {
            result = Some(context.1.clone());
        }
    });
    return result;
}

fn clear_di_context_id(id: &InstanceId) {
    DI_REGISTRY.with(|di_registry| {
        di_registry.borrow_mut().remove(id);
    });
}

fn clear_di_context(context: &Gd<DiContext>) {
    let mut id_to_clear = None;
    DI_REGISTRY.with(|di_registry| {
        for (key, value) in di_registry.borrow().iter() {
            if value.1 == *context {
                id_to_clear = Some(*key);
                break;
            }
        }
    });
    if let Some(id_to_clear) = id_to_clear {
        clear_di_context_id(&id_to_clear);
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct DiContext {
    parent_context: Option<Gd<DiContext>>,
    registered_nodes: HashMap<(GodotString, GodotString), Gd<Node>>,

    #[base]
    base: Base<Node>,
}

impl DiContext {
    pub fn get_registered_node_template<T: GodotClass + Inherits<Node>>(
        &self,
        id: GodotString,
    ) -> Option<Gd<T>> {
        self.get_registered_node_with_id(T::class_name().to_godot_string(), id)
            .map(|node| node.cast())
    }
}

#[godot_api]
impl DiContext {
    #[func]
    pub fn get_registered_node(&self, type_name: GodotString) -> Option<Gd<Node>> {
        return self.get_registered_node_with_id(type_name, "".into());
    }

    #[func]
    pub fn get_registered_node_with_id(
        &self,
        type_name: GodotString,
        id: GodotString,
    ) -> Option<Gd<Node>> {
        if let Some(locally_found) = self.registered_nodes.get(&(type_name.clone(), id.clone())) {
            return Some(locally_found.clone());
        } else {
            if let Some(parent_context) = self.parent_context.as_ref() {
                return parent_context
                    .bind()
                    .get_registered_node_with_id(type_name, id);
            } else {
                return None;
            }
        }
    }

    #[func]
    pub fn register_node_of_type(
        &mut self,
        node: Gd<Node>,
        type_name: GodotString,
        id: GodotString,
    ) {
        self.registered_nodes.insert((type_name, id), node);
    }

    #[func]
    pub fn register_node(&mut self, mut node: Gd<Node>, id: GodotString) {
        let type_name;
        let custom_lookup_method: StringName = "_di_name".into();
        if node.has_method(custom_lookup_method.clone()) {
            type_name = node.call(custom_lookup_method, &[]).stringify();
        } else {
            type_name = node.get_class();
        }
        self.register_node_of_type(node, type_name, id);
    }

    #[func]
    pub fn get_context(node: Gd<Node>) -> Option<Gd<DiContext>> {
        lookup_di_context(&node)
    }

    #[func]
    pub fn get_nearest(node: Gd<Node>) -> Option<Gd<DiContext>> {
        if let Some(context) = lookup_di_context(&node) {
            return Some(context);
        } else {
            if let Some(parent) = node.get_parent() {
                return Self::get_nearest(parent);
            } else {
                return None;
            }
        }
    }

    #[func]
    pub fn get_nearest_exclude_self(node: Gd<Node>) -> Option<Gd<DiContext>> {
        Self::get_nearest(node.get_parent().unwrap())
    }
}

#[godot_api]
impl NodeVirtual for DiContext {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            parent_context: None,
            registered_nodes: Default::default(),
            base,
        }
    }

    fn ready(&mut self) {}

    fn enter_tree(&mut self) {
        let parent = self.base.get_parent().unwrap();
        insert_di_context(parent.clone(), self.base.clone().cast());
        self.parent_context = Self::get_nearest_exclude_self(parent);
    }

    fn exit_tree(&mut self) {
        clear_di_context(&self.base.clone().cast());
    }
}

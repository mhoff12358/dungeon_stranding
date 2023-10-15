use godot::prelude::*;

use super::di_context::DiContext;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct DiRegistration {
    #[export]
    id: GodotString,

    #[export]
    remove_registration_object: bool,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl DiRegistration {}

#[godot_api]
impl NodeVirtual for DiRegistration {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            id: "".into(),
            remove_registration_object: false,
            base,
        }
    }

    fn enter_tree(&mut self) {
        let parent = self.base.get_parent().unwrap();
        if let Some(mut context) = DiContext::get_nearest_exclude_self(parent.clone()) {
            context
                .bind_mut()
                .register_node(parent.clone(), self.id.clone());
        } else {
            godot_print!("Tried to register a node with no context in its parentage.");
        }
        self.base.queue_free();
    }
}

use godot::{obj::WithBaseField, prelude::*};

pub fn upcast_base<T: GodotClass, U: GodotClass>(upcasted: Gd<T>) -> Gd<U>
where
    Gd<T>: WithBaseField + Inherits<U>,
{
    upcasted.to_gd().upcast::<U>()
}

pub fn try_walk_parents_for<T: Inherits<Node>>(node: &Node) -> Option<Gd<T>> {
    let mut parent_walk = node.get_parent();
    while let Some(parent) = &parent_walk {
        if let Ok(game_state) = parent.clone().try_cast() {
            return Some(game_state);
        }
        parent_walk = parent.get_parent();
    }
    return None;
}

/*pub fn walk_parents_for<T: Inherits<Node>>(node: &Node) -> Gd<T> {
    return try_walk_parents_for(node).unwrap();
}*/
pub fn walk_parents_for<T: Inherits<Node>, U: GodotClass + Inherits<Node>>(node: &Gd<U>) -> Gd<T> {
    return try_walk_parents_for(&node.clone().upcast::<Node>()).unwrap();
}

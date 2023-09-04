use godot::prelude::*;

pub fn try_walk_parents_for<T: Inherits<Node>>(node: &Node) -> Option<Gd<T>> {
    let mut parent_walk = node.get_parent();
    while let Some(parent) = &parent_walk {
        if let Some(game_state) = parent.share().try_cast() {
            return Some(game_state);
        }
        parent_walk = parent.get_parent();
    }
    return None;
}

pub fn walk_parents_for<T: Inherits<Node>>(node: &Node) -> Gd<T> {
    return try_walk_parents_for(node).unwrap();
}

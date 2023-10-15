use godot::prelude::*;

pub fn pack_node(node: Gd<Node>) -> Gd<PackedScene> {
    let mut packed_scene = PackedScene::new();
    set_childrens_owner(&node, &node);
    packed_scene.pack(node);
    return packed_scene;
}

fn set_childrens_owner(node: &Gd<Node>, owner: &Gd<Node>) {
    let children = node.get_children_ex().include_internal(true).done();
    for mut child in children.iter_shared() {
        child.set_owner(owner.clone());
        set_childrens_owner(&child, owner);
    }
}

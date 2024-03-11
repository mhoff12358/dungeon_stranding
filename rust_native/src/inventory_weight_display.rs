use ds_lib::game_state::items::inventory::Inventory;
use godot::{
    engine::{global::Side, Control, Label},
    prelude::*,
};

pub fn update_weight_display(
    inventory: &Inventory,
    text: &mut Gd<Label>,
    bar_foreground: &mut Gd<Control>,
) {
    let total_weight = inventory.total_weight();
    if let Some(weight_capacity) = inventory.weight_capacity() {
        text.set_text(format!("{}/{}", total_weight, weight_capacity,).into());
        bar_foreground
            .set_anchor_ex(
                Side::RIGHT,
                f32::min(1.0, total_weight as f32 / weight_capacity as f32),
            )
            .keep_offset(true)
            .done();
    } else {
        text.set_text(format!("{}", total_weight).into());
        bar_foreground
            .set_anchor_ex(Side::RIGHT, 0.0)
            .keep_offset(true)
            .done();
    };
}

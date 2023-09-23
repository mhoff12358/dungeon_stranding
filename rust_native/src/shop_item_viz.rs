use std::ops::Deref;

use ds_lib::{party_state::inventory::ItemInfo, shop::shop::Shop};
use godot::{
    engine::{global::Side, Control, ControlVirtual, Label},
    prelude::*,
};
use owning_ref::OwningHandle;

use crate::{
    my_gd_ref::MyGdRef,
    shop_viz::{ShopId, ShopViz},
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ShopItemViz {
    shop: Option<Gd<ShopViz>>,

    shop_id: Option<ShopId>,
    #[export]
    item_name_label: Option<Gd<Label>>,
    #[export]
    price_label: Option<Gd<Label>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl ShopItemViz {
    #[signal]
    fn instantiate_template(&self, value: Variant);

    #[signal]
    fn place_after(&self, previous: Variant);

    #[func]
    fn _on_instantiate_template(&mut self, value: Variant) {
        let shop_id: ShopId;
        let item_text: GodotString;
        let price_text: GodotString;
        {
            shop_id = ShopId::from_variant(&value);
            let shop = self.shop();
            let item = shop.get_item(&shop_id.0).unwrap();
            item_text = item.item.name().into();
            price_text = format!("{}g", item.price).into();
        }
        self.shop_id = Some(shop_id);
        self.item_name_label.as_mut().unwrap().set_text(item_text);
        self.price_label.as_mut().unwrap().set_text(price_text);
    }

    #[func]
    fn _on_place_after(&mut self, previous: Variant) {
        let top;
        if previous.is_nil() {
            top = 0.0;
        } else {
            let previous_control = Gd::<Control>::from_variant(&previous);
            top = previous_control.get_anchor(Side::SIDE_BOTTOM);
        }

        let size = self.base.get_anchor(Side::SIDE_BOTTOM) - self.base.get_anchor(Side::SIDE_TOP);

        self.base
            .set_anchor_ex(Side::SIDE_TOP, top)
            .keep_offset(true)
            .done();
        self.base
            .set_anchor_ex(Side::SIDE_BOTTOM, top + size)
            .keep_offset(true)
            .done();
    }

    #[func(gd_self)]
    fn buy_item(this: Gd<Self>) {
        let shop: Gd<ShopViz>;
        let item_to_buy: ShopId;
        {
            let _self = this.bind();
            shop = _self.shop.as_ref().unwrap().clone();
            item_to_buy = _self.shop_id.unwrap();
        }
        ShopViz::buy_item(shop, item_to_buy);
    }
}

impl ShopItemViz {
    pub fn shop<'a>(&'a self) -> impl Deref<Target = Shop> + 'a {
        OwningHandle::new_with_fn(
            MyGdRef::new(self.shop.as_ref().unwrap().bind()),
            |it: *const ShopViz| {
                let it = unsafe { &*it };
                it.shop()
            },
        )
    }
}

#[godot_api]
impl ControlVirtual for ShopItemViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            shop: None,
            shop_id: None,
            item_name_label: None,
            price_label: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        {
            let callable = self.base.callable("_on_instantiate_template");
            self.base.connect("instantiate_template".into(), callable);
        }
        {
            let callable = self.base.callable("_on_place_after");
            self.base.connect("place_after".into(), callable);
        }
    }

    fn ready(&mut self) {
        self.shop = Some(walk_parents_for(&self.base));
    }
}

use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use ds_lib::{
    game_state::game_state::GameState,
    out_of_dungeon_algos::purchase_from_shop,
    party_state::inventory::UniqueItemId,
    shop::{shop::Shop, shop_interface::ShopInterface},
};
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};
use owning_ref::{OwningHandle, OwningRef, OwningRefMut};

use crate::{
    game_state_viz::{borrow_game_state, borrow_game_state_mut, GameStateViz},
    out_of_dungeon_viz::OutOfDungeonViz,
    template_spawner::TemplateSpawner,
    tree_utils::walk_parents_for,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ShopId(pub UniqueItemId);

impl ToVariant for ShopId {
    fn to_variant(&self) -> Variant {
        self.0 .0.to_variant()
    }
}

impl FromVariant for ShopId {
    fn try_from_variant(variant: &Variant) -> Result<Self, VariantConversionError> {
        Ok(ShopId(UniqueItemId(u32::try_from_variant(variant)?)))
    }
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ShopViz {
    game_state: Option<Gd<GameStateViz>>,
    out_of_dungeon: Option<Gd<OutOfDungeonViz>>,

    #[export]
    shop_item_template: Option<Gd<Control>>,
    shop_item_templates: Option<RefCell<TemplateSpawner<ShopId>>>,

    #[base]
    base: Base<Control>,
}

impl ShopViz {
    pub fn shop<'a>(&'a self) -> impl Deref<Target = Shop> + 'a {
        OwningRef::new(borrow_game_state(&self.game_state.as_ref().unwrap()))
            .map(|game_state| &game_state.unwrap_out_of_dungeon().shop)
    }

    pub fn shop_mut<'a>(&'a self) -> impl DerefMut<Target = Shop> + 'a {
        OwningRefMut::new(borrow_game_state_mut(
            &mut self.game_state.as_ref().unwrap(),
        ))
        .map_mut(|game_state| &mut game_state.unwrap_out_of_dungeon_mut().shop)
    }

    pub fn shop_interface<'a>(&'a self) -> impl Deref<Target = ShopInterface> + 'a {
        OwningHandle::new_with_fn(
            borrow_game_state(&self.game_state.as_ref().unwrap()),
            |it: *const GameState| {
                let it = unsafe { &*it };
                &it.unwrap_out_of_dungeon().shop_interface
            },
        )
    }

    pub fn buy_item(mut shop_viz: Gd<ShopViz>, item_to_buy: ShopId) {
        let game_state: Gd<GameStateViz>;
        {
            let _self = shop_viz.bind_mut();
            game_state = _self.game_state();
            let mut game_state = borrow_game_state_mut(&game_state);
            purchase_from_shop(game_state.unwrap_out_of_dungeon_mut(), item_to_buy.0);
        }
        GameStateViz::handle_game_update(game_state);
    }
}

#[godot_api]
impl ShopViz {
    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().clone()
    }

    #[func]
    pub fn out_of_dungeon(&self) -> Gd<OutOfDungeonViz> {
        self.out_of_dungeon.as_ref().unwrap().clone()
    }

    #[func]
    pub fn _on_out_of_dungeon_state_updated(&self) {
        let shop = self.shop();
        self.shop_item_templates
            .as_ref()
            .unwrap()
            .borrow_mut()
            .update(shop.display_order().iter().map(|id| ShopId(*id)), |x| *x);
    }
}

#[godot_api]
impl ControlVirtual for ShopViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            out_of_dungeon: None,
            shop_item_template: None,
            shop_item_templates: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(walk_parents_for(&self.base));
        self.out_of_dungeon = Some(walk_parents_for(&self.base));
        self.out_of_dungeon.as_mut().unwrap().connect(
            "updated_state".into(),
            self.base.callable("_on_out_of_dungeon_state_updated"),
        );
        self.shop_item_templates = Some(RefCell::new(TemplateSpawner::new(
            self.shop_item_template.as_ref().unwrap().clone().upcast(),
        )));
    }
}

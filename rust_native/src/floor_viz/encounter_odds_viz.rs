use std::ops::Deref;

use ds_lib::{
    coord::Coord,
    dungeon_state::{
        tile_entity::TileEntity,
        tile_state::{OpenType, SpecificTS, TileState},
    },
    game_state::game_state::InDungeon,
};
use godot::{
    engine::{Control, ControlVirtual, TileMap},
    prelude::*,
};

use crate::{in_dungeon_viz::InDungeonViz, tree_utils::walk_parents_for};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct EncoutnerOddsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl EncoutnerOddsViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }
}

#[godot_api]
impl ControlVirtual for EncoutnerOddsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            self.base.callable("_on_in_dungeon_updated"),
        );
    }
}

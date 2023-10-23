use std::ops::Deref;

use ds_lib::{
    coord::Coord,
    dungeon_state::{
        encounters::wandering_encounters::WanderingEncounterOdds,
        tile_entity::TileEntity,
        tile_state::{OpenType, SpecificTS, TileState},
        zone::ZoneId,
    },
    game_state::game_state::InDungeon,
};
use godot::{
    engine::{Control, ControlVirtual, TileMap},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext, in_dungeon_viz::InDungeonViz,
    template_spawners::template_spawner::TemplateSpawner, tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EncoutnerOddsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    //spawner: Option<TemplateSpawner<ZoneId, (ZoneId, WanderingEncounterOdds), (), Node2D>>,
    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl EncoutnerOddsViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }
}

#[godot_api]
impl Node2DVirtual for EncoutnerOddsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,

            //spawner: None,
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

    fn ready(&mut self) {
        let di_context = DiContext::get_nearest(self.base.clone().upcast()).unwrap();
        let di_context = di_context.bind();
    }
}

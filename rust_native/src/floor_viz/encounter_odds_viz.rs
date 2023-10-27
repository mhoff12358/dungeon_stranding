use std::{cell::RefCell, ops::Deref};

use ds_lib::{
    coord::Coord,
    dungeon_state::{
        encounters::{encounter_risk::EncounterRisk, wandering_encounters::WanderingEncounterOdds},
        floor_layout::FloorLayout,
        tile_entity::TileEntity,
        tile_state::{OpenType, SpecificTS, TileState},
        zone::{Zone, ZoneId},
    },
    game_state::game_state::InDungeon,
};
use godot::{
    engine::{ColorRect, Control, ControlVirtual, TileMap},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    floor_viz::floor_layout_viz::FloorLayoutViz,
    in_dungeon_viz::InDungeonViz,
    template_spawners::{
        template_spawner::{TemplateGenerics, TemplateSpawner},
        update_behavior::TemplateSpawnerUpdateBehavior,
    },
    tree_utils::walk_parents_for,
};

use super::tile_spacing::TileSpacing;

pub struct WanderingEncounterOddsGenerics {}

impl TemplateGenerics for WanderingEncounterOddsGenerics {
    type Key = ZoneId;
    type Value = (ZoneId, WanderingEncounterOdds);
    type Context = RefCell<Gd<EncounterOddsViz>>;
    type TemplateType = ColorRect;
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EncounterOddsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    spawner: Option<TemplateSpawner<WanderingEncounterOddsGenerics, Self>>,

    tile_map: Option<Gd<TileMap>>,

    #[base]
    base: Base<Node2D>,
}

#[godot_api]
impl EncounterOddsViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {}
}

impl TemplateSpawnerUpdateBehavior for EncounterOddsViz {
    type Generics = WanderingEncounterOddsGenerics;

    fn initialize(
        mut template: Gd<ColorRect>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &RefCell<Gd<EncounterOddsViz>>,
    ) {
        let mut self_ptr = context.borrow_mut();
        let self_ = self_ptr.bind_mut();
        let tile_spacing = TileSpacing::new(self_.tile_map.as_ref().unwrap());

        let in_dungeon = self_.in_dungeon.as_ref().unwrap().bind();
        let current_floor = in_dungeon.borrow_current_floor();

        let zones = current_floor.layout().zones();

        let zone = zones.get(&value.0).unwrap();
        match zone {
            Zone::Room { room, .. } => {
                let room_center = Coord::new(
                    (room.bounds.min.x + room.bounds.max.x) / 2,
                    (room.bounds.min.y + room.bounds.max.y) / 2,
                );

                template.set_position(tile_spacing.entity_position(room_center));
                template.set_size(Vector2::from_vector2i(tile_spacing.tile_size()));
            }
            _ => {
                template.set_visible(false);
            }
        }
    }

    fn place_after(
        mut template: Gd<ColorRect>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        let mut self_ptr = context.borrow_mut();
        let self_ = self_ptr.bind_mut();

        let in_dungeon = self_.in_dungeon.as_ref().unwrap().bind();
        let current_floor = in_dungeon.borrow_current_floor();

        let wandering_encounters = current_floor.wandering_encounters();

        let encounter_odds = wandering_encounters.get_encounter_odds(value.0);

        let amount = 0.5;
        template.set_color(Color::from_rgba(
            f32::lerp(0.0, 1.0, amount),
            f32::lerp(1.0, 0.0, amount),
            0.0,
            0.2,
        ));
    }
}

#[godot_api]
impl Node2DVirtual for EncounterOddsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            spawner: None,
            tile_map: None,

            base,
        }
    }

    fn ready(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            self.base.callable("_on_in_dungeon_updated"),
        );
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            self.base.callable("_on_in_dungeon_updated"),
        );

        let di_context = DiContext::get_nearest(self.base.clone().upcast()).unwrap();
        let di_context = di_context.bind();
        let template = di_context.get_registered_node_template::<ColorRect>("template".into());
        self.tile_map = Some(di_context.get_registered_node_template::<TileMap>("".into()));
        self.spawner = Some(TemplateSpawner::new(template));
    }
}

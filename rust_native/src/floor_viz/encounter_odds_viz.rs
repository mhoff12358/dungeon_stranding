use ds_lib::{
    coord::Coord,
    dungeon_state::{
        encounters::{
            wandering_encounter_odds::WanderingEncounterOdds, wandering_encounters::OddsType,
        },
        zone::{Zone, ZoneId},
    },
};
use godot::{
    engine::{ColorRect, Sprite2D, TileMap},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    in_dungeon_viz::InDungeonViz,
    template_spawners::{
        template_spawner::{TemplateGenerics, TemplateSpawner},
        update_behavior::TemplateSpawnerUpdateBehavior,
    },
    tree_utils::walk_parents_for,
};

use super::tile_spacing::TileSpacing;

pub struct WanderingEncounterOddsGenerics {}

pub struct Context {
    in_dungeon: Gd<InDungeonViz>,
    tile_map: Gd<TileMap>,
}

impl TemplateGenerics for WanderingEncounterOddsGenerics {
    type Key = ZoneId;
    type Value = (ZoneId, WanderingEncounterOdds);
    type Context = Context;
    type TemplateType = Node2D;
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct EncounterOddsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    spawner: Option<TemplateSpawner<WanderingEncounterOddsGenerics, Self>>,

    tile_map: Option<Gd<TileMap>>,

    base: Base<Node2D>,
}

#[godot_api]
impl EncounterOddsViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func(gd_self)]
    pub fn _on_in_dungeon_updated(mut this: Gd<Self>) {
        let mut self_ = this.bind_mut();
        let zones_with_odds: Vec<_>;
        {
            let in_dungeon = self_.in_dungeon.as_ref().unwrap().bind();
            let current_floor = in_dungeon.borrow_current_floor();
            let zones = current_floor.layout().zones().keys().cloned();
            zones_with_odds = zones
                .map(|zone_id| {
                    (
                        zone_id,
                        current_floor
                            .wandering_encounters()
                            .get_encounter_odds(zone_id, OddsType::ExcludeRecentEncounters),
                    )
                })
                .collect();
        }
        let context = Context {
            tile_map: self_.tile_map.clone().unwrap(),
            in_dungeon: self_.in_dungeon.clone().unwrap(),
        };
        self_.spawner.as_mut().unwrap().update_with_getter(
            zones_with_odds.iter(),
            |x| x.0,
            &context,
        );
    }
}

impl TemplateSpawnerUpdateBehavior for EncounterOddsViz {
    type Generics = WanderingEncounterOddsGenerics;

    fn initialize(
        mut template: Gd<Node2D>,
        value: &(ZoneId, WanderingEncounterOdds),
        context: &Context,
    ) {
        let tile_spacing = TileSpacing::new(&context.tile_map);

        let in_dungeon = context.in_dungeon.bind();
        let current_floor = in_dungeon.borrow_current_floor();

        let zones = current_floor.layout().zones();

        let zone = zones.get(&value.0).unwrap();
        let display_coord;
        match zone {
            Zone::Room { room, .. } => {
                display_coord = Some(Coord::new(
                    (room.bounds.min.x + room.bounds.max.x) / 2,
                    (room.bounds.min.y + room.bounds.max.y) / 2,
                ));
            }
            Zone::Hallway { hallway, .. } => {
                if !hallway.coords.is_empty() {
                    display_coord = Some(hallway.coords[0]);
                } else {
                    display_coord = None;
                }
            }
        }

        if let Some(display_coord) = display_coord {
            template.set_position(tile_spacing.entity_position(display_coord));
        } else {
            template.set_visible(false);
        }
    }

    fn update_template(
        template: Gd<Node2D>,
        value: &<Self::Generics as TemplateGenerics>::Value,
        context: &<Self::Generics as TemplateGenerics>::Context,
        _previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        let in_dungeon = context.in_dungeon.bind();
        let current_floor = in_dungeon.borrow_current_floor();

        let wandering_encounters = current_floor.wandering_encounters();

        let encounter_probability = wandering_encounters
            .get_encounter_odds(value.0, OddsType::ExcludeRecentEncounters)
            .combined_probability();

        let encounters_in_zone = wandering_encounters.get_encounters_in_zone(value.0);

        let context = DiContext::get_nearest_bound(template);
        let mut background_color: Gd<ColorRect> =
            context.get_registered_node_template("background_color".into());
        let mut sprite: Gd<Sprite2D> = context.get_registered_node_template("".into());

        background_color.set_color(Color::from_rgba(
            f32::lerp(0.0, 1.0, encounter_probability),
            f32::lerp(1.0, 0.0, encounter_probability),
            0.0,
            0.5,
        ));
        if encounter_probability <= 0.0 {
            background_color.set_visible(false);
        }
        sprite.set_visible(encounters_in_zone.is_some());
    }
}

#[godot_api]
impl INode2D for EncounterOddsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            spawner: None,
            tile_map: None,

            base,
        }
    }

    fn ready(&mut self) {
        let gd_self = self.to_gd();

        self.in_dungeon = Some(walk_parents_for(&gd_self));
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            gd_self.callable("_on_in_dungeon_updated"),
        );

        let di_context = DiContext::get_nearest(self.base().clone().upcast()).unwrap();
        let di_context = di_context.bind();
        let template = di_context.get_registered_node_template::<Node2D>("template".into());
        self.tile_map = Some(di_context.get_registered_node_template::<TileMap>("".into()));
        self.spawner = Some(TemplateSpawner::new(template));
    }
}

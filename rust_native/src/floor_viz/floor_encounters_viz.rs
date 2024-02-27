use std::collections::HashMap;

use ds_lib::bestiary::monster_stats::MonsterStats;
use godot::{
    engine::{Control, IControl},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    in_dungeon_viz::InDungeonViz,
    template_spawners::{
        template_spawner::{TemplateGenerics, TemplateSpawner},
        update_behavior::ValueTransformedSignals,
    },
    tree_utils::walk_parents_for,
};

pub struct AllFloorEncountersGenerics {}

impl TemplateGenerics for AllFloorEncountersGenerics {
    type Key = String;
    type Value = (MonsterStats, i32);
    type Context = ();
    type TemplateType = Control;
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct FloorEncountersViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    spawner: Option<TemplateSpawner<AllFloorEncountersGenerics, Self>>,

    base: Base<Control>,
}

impl ValueTransformedSignals for FloorEncountersViz {
    type Generics = AllFloorEncountersGenerics;

    fn transform_value(
        _context: &<Self::Generics as TemplateGenerics>::Context,
        value: &(MonsterStats, i32),
    ) -> Vec<Variant> {
        return vec![value.0.name.to_variant(), value.1.to_variant()];
    }
}

#[godot_api]
impl FloorEncountersViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func(gd_self)]
    pub fn _on_in_dungeon_updated(mut this: Gd<Self>) {
        let mut self_ = this.bind_mut();
        let mut tabulated_encounters: HashMap<String, (MonsterStats, i32)> = HashMap::new();
        {
            let in_dungeon = self_.in_dungeon.as_ref().unwrap().bind();
            let current_floor = in_dungeon.borrow_current_floor();
            let encounters = current_floor.wandering_encounters.get_encounters();
            for (_zone_id, encounter) in encounters {
                let stats = encounter.borrow().monster.clone();
                tabulated_encounters
                    .entry(stats.name.clone())
                    .or_insert((stats, 0))
                    .1 += 1;
            }
        }
        self_.spawner.as_mut().unwrap().update_with_getter(
            tabulated_encounters.values(),
            |x| x.0.name.clone(),
            &(),
        );
    }
}

#[godot_api]
impl IControl for FloorEncountersViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            spawner: None,

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
        let template = di_context.get_registered_node_template::<Control>("template".into());
        self.spawner = Some(TemplateSpawner::new(template));
    }
}

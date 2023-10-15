use ds_lib::game_state::game_state::OngoingInteraction;
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};

use crate::{
    camp_viz::CampViz,
    di_context::di_context::DiContext,
    dig_viz::DigViz,
    game_state_viz::GameStateViz,
    in_dungeon_viz::InDungeonViz,
    loot_viz::{self, loot_viz::LootViz},
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InteractionViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    dig_viz: Option<Gd<DigViz>>,
    camp_viz: Option<Gd<CampViz>>,
    loot_viz: Option<Gd<LootViz>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl InteractionViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func]
    pub fn hide(&mut self) {
        self.base.set_visible(false);
    }

    #[func]
    pub fn _on_in_dungeon_updated_interaction(&mut self) {
        self.base.set_visible(true);
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let interaction = in_dungeon
            .ongoing_event
            .as_ref()
            .unwrap()
            .unwrap_interaction();
        match interaction {
            OngoingInteraction::Dig => {
                self.dig_viz.as_mut().unwrap().bind_mut().updated();
            }
            OngoingInteraction::Camp { amount } => {
                self.camp_viz.as_mut().unwrap().bind_mut().updated(amount);
            }
            OngoingInteraction::Loot(..) => {
                self.loot_viz.as_mut().unwrap().bind_mut().updated();
            }
        }
    }
}

#[godot_api]
impl ControlVirtual for InteractionViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,

            camp_viz: None,
            dig_viz: None,
            loot_viz: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        walk_parents_for::<GameStateViz>(&self.base)
            .connect("pre_updated_state".into(), self.base.callable("hide"));
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            "updated_state_interaction".into(),
            self.base.callable("_on_in_dungeon_updated_interaction"),
        );
    }

    fn ready(&mut self) {
        self.dig_viz = DiContext::get_nearest(self.base.clone().upcast())
            .unwrap()
            .bind()
            .get_registered_node_template("".into());
        self.camp_viz = DiContext::get_nearest(self.base.clone().upcast())
            .unwrap()
            .bind()
            .get_registered_node_template("".into());
        self.loot_viz = DiContext::get_nearest(self.base.clone().upcast())
            .unwrap()
            .bind()
            .get_registered_node_template("".into());
    }
}

use ds_lib::game_state::game_state::OngoingInteraction;
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};

use crate::{
    camp_viz::CampViz, dig_viz::DigViz, game_state_viz::GameStateViz, in_dungeon_viz::InDungeonViz,
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InteractionViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    pub dig_viz: Option<Gd<DigViz>>,
    pub camp_viz: Option<Gd<CampViz>>,

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
                if let Some(dig_viz) = self.dig_viz.as_mut() {
                    dig_viz.bind_mut().updated();
                }
            }
            OngoingInteraction::Camp { amount } => {
                if let Some(camp_viz) = self.camp_viz.as_mut() {
                    camp_viz.bind_mut().updated(amount);
                }
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
}

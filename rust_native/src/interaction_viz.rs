use ds_lib::{
    fight::fight::{Fight, FightPhase},
    game_state::{game_state::OngoingInteraction, state_updates::interactions::Interaction},
};
use godot::{
    engine::{Control, ControlVirtual, Label},
    prelude::*,
};

use crate::{in_dungeon_viz::InDungeonViz, tree_utils::walk_parents_for};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InteractionViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    interaction_options_label: Option<Gd<Label>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl InteractionViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().share()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {
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

        self.interaction_options_label
            .as_mut()
            .unwrap()
            .set_text(Self::get_interaction_description(interaction));
    }
}

impl InteractionViz {
    fn get_interaction_description(interaction: &OngoingInteraction) -> GodotString {
        let text = match interaction {
            OngoingInteraction::Dig => {
                format!("Pick a direction to dig\nQ to quit")
            }
            OngoingInteraction::Camp { amount } => {
                format!(
                    "How much food to spend camping? (A & D to change)\n{} food\n\nSpace to finalize, Q to quit",
                    amount
                )
            }
        };
        return text.into();
    }
}

#[godot_api]
impl ControlVirtual for InteractionViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            interaction_options_label: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            "updated_state".into(),
            self.base.callable("_on_in_dungeon_updated"),
        );
        self.in_dungeon.as_mut().unwrap().connect(
            "updated_state_interaction".into(),
            self.base.callable("_on_in_dungeon_updated_interaction"),
        );
    }
}

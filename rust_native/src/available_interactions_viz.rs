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
pub struct AvailableInteractionsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    interactions_label: Option<Gd<Label>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl AvailableInteractionsViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().share()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {
        self.base.set_visible(true);
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();

        self.interactions_label
            .as_mut()
            .unwrap()
            .set_text(Self::get_interactions(&in_dungeon.interactions));
    }
}

impl AvailableInteractionsViz {
    fn get_interactions(interactions: &Vec<Interaction>) -> GodotString {
        let mut text = String::new();
        for (index, interaction) in interactions.iter().enumerate() {
            text = format!("{}{}: {}\n", text, index + 1, interaction.description());
        }
        return text.into();
    }
}

#[godot_api]
impl ControlVirtual for AvailableInteractionsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            interactions_label: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            "updated_state".into(),
            self.base.callable("_on_in_dungeon_updated"),
        );
    }
}

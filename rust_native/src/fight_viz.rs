use ds_lib::{
    fight::fight::{Fight, FightPhase},
    game_state::inputs::fight_input::FightInput,
};
use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};

use crate::{
    game_state_viz::GameStateViz, in_dungeon_viz::InDungeonViz, tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct FightViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    fight_description_label: Option<Gd<Label>>,

    base: Base<Control>,
}

#[godot_api]
impl FightViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {
        self.base_mut().set_visible(false);
    }

    #[func]
    pub fn _on_in_dungeon_updated_fight(&mut self) {
        self.base_mut().set_visible(true);
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let fight = in_dungeon.ongoing_event.as_ref().unwrap().unwrap_fight();

        self.fight_description_label
            .as_mut()
            .unwrap()
            .set_text(Self::get_fight_description(fight));
    }

    #[func(gd_self)]
    pub fn advance_fight(this: Gd<Self>) {
        let game_state = this.bind().in_dungeon.as_ref().unwrap().bind().game_state();
        GameStateViz::accept_input(game_state, &FightInput::advance());
    }
}

impl FightViz {
    fn get_fight_description(fight: &Fight) -> GString {
        let fight_description = match fight.phase() {
            FightPhase::Pre => {
                format!("You are attacked by a {}.", fight.enemy().name.as_str())
            }
            FightPhase::Resolved {
                start_damage,
                total_damage,
                equipment_effects,
            } => {
                let mut result = format!(
                    "You are attacked by a {}.\nIt does {} damage.",
                    fight.enemy().name.as_str(),
                    start_damage
                );
                for (name, effect) in equipment_effects.iter() {
                    result = format!(
                        "{}\nYour {} reduces it by {} damage.",
                        result,
                        name.as_str(),
                        effect
                    );
                }
                format!("{}\nYou take {} damage", result, total_damage)
            }
        };
        return fight_description.into();
    }
}

#[godot_api]
impl IControl for FightViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            fight_description_label: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.to_gd()));
        let _on_in_dungeon_updated = self.base().callable("_on_in_dungeon_updated");
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            _on_in_dungeon_updated,
        );
        let _on_in_dungeon_updated_fight = self.base().callable("_on_in_dungeon_updated_fight");
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_FIGHT_SIGNAL.into(),
            _on_in_dungeon_updated_fight,
        );
    }
}

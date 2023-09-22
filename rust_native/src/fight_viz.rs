use ds_lib::fight::fight::{Fight, FightPhase};
use godot::{
    engine::{Control, ControlVirtual, Label},
    prelude::*,
};

use crate::{in_dungeon_viz::InDungeonViz, tree_utils::walk_parents_for};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct FightViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    fight_description_label: Option<Gd<Label>>,

    #[base]
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
        self.base.set_visible(false);
    }

    #[func]
    pub fn _on_in_dungeon_updated_fight(&mut self) {
        self.base.set_visible(true);
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let fight = in_dungeon.ongoing_event.as_ref().unwrap().unwrap_fight();

        self.fight_description_label
            .as_mut()
            .unwrap()
            .set_text(Self::get_fight_description(fight));
    }
}

impl FightViz {
    fn get_fight_description(fight: &Fight) -> GodotString {
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
impl ControlVirtual for FightViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            fight_description_label: None,

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
            "updated_state_fight".into(),
            self.base.callable("_on_in_dungeon_updated_fight"),
        );
    }
}

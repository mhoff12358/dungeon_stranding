use ds_lib::game_state::{
    game_state::OngoingInteraction,
    inputs::{
        dig_input::DigInput,
        open_close_door_input::{OpenCloseDoorInput, OpenCloseDoorIntent},
        piton_input::{PitonDoorInput, PitonDoorIntent},
    },
    state_updates::interactions::{add_remove_description, opening_description},
};
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    game_state_viz::GameStateViz,
    in_dungeon_viz::InDungeonViz,
    interactions_viz::{
        camp_viz::CampViz,
        direction_picker_viz::{DirectionPickerConfig, DirectionPickerViz},
    },
    loot_viz::loot_viz::LootViz,
    tree_utils::walk_parents_for,
};

struct Visualizers {
    camp: Gd<CampViz>,
    loot: Gd<LootViz>,
    direction: Gd<DirectionPickerViz>,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InteractionViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    viz: Option<Visualizers>,

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
            OngoingInteraction::Dig { directions } => {
                let display_text = format!("Dig through a wall");
                self.viz.as_mut().unwrap().direction.bind_mut().updated(
                    display_text,
                    DirectionPickerConfig {
                        cancel_input: Some(DigInput::cancel()),
                        directed_input: Box::new(|direction| DigInput::do_dig(direction)),
                        allowed_directions: directions.clone(),
                    },
                );
            }
            OngoingInteraction::Camp { amount } => {
                self.viz.as_mut().unwrap().camp.bind_mut().updated(amount);
            }
            OngoingInteraction::Loot(..) => {
                self.viz.as_mut().unwrap().loot.bind_mut().updated();
            }
            OngoingInteraction::OpenCloseDoor { open, directions } => {
                let display_text = format!("{} a door", opening_description(*open));
                let open = *open;
                self.viz.as_mut().unwrap().direction.bind_mut().updated(
                    display_text,
                    DirectionPickerConfig {
                        cancel_input: Some(OpenCloseDoorInput::cancel()),
                        directed_input: Box::new(move |direction| {
                            OpenCloseDoorInput::open_close(OpenCloseDoorIntent {
                                open: open,
                                direction: direction,
                            })
                        }),
                        allowed_directions: directions.clone(),
                    },
                );
            }
            OngoingInteraction::PitonDoor { insert, directions } => {
                let display_text = format!("{} a door", add_remove_description(*insert));
                let insert = *insert;
                self.viz.as_mut().unwrap().direction.bind_mut().updated(
                    display_text,
                    DirectionPickerConfig {
                        cancel_input: Some(PitonDoorInput::cancel()),
                        directed_input: Box::new(move |direction| {
                            PitonDoorInput::add_remove(PitonDoorIntent {
                                insert: insert,
                                direction: direction,
                            })
                        }),
                        allowed_directions: directions.clone(),
                    },
                );
            }
        }
    }
}

#[godot_api]
impl ControlVirtual for InteractionViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,

            viz: None,

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
        let context = DiContext::get_nearest_bound(self.base.clone());
        self.viz = Some(Visualizers {
            camp: context.get_registered_node_template("".into()),
            loot: context.get_registered_node_template("".into()),
            direction: context.get_registered_node_template("".into()),
        });
    }
}

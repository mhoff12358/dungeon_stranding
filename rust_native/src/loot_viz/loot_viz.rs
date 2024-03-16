use ds_lib::game_state::{
    game_state::{GameState, InDungeon, InDungeonEvent, OngoingInteraction},
    inputs::loot_input::LootInput,
};
use godot::{
    engine::{Control, IControl},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    game_state_viz::{borrow_game_state, GameStateViz},
    tree_utils::walk_parents_for,
};

use super::ongoing_transfer_viz::OngoingTransferViz;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct LootViz {
    game_state: Option<Gd<GameStateViz>>,

    ongoing_transfer_viz: Option<Gd<OngoingTransferViz>>,

    base: Base<Control>,
}

#[godot_api]
impl LootViz {
    fn game_in_looting_interaction(&self, game_state: &GameState) -> bool {
        return match game_state {
            GameState::InDungeon(InDungeon {
                ongoing_event: Some(InDungeonEvent::Interaction(OngoingInteraction::Loot { .. })),
                ..
            }) => true,
            _ => false,
        };
    }

    #[func(gd_self)]
    pub fn pre_updated(mut this: Gd<Self>) {
        let mut _self = this.bind_mut();

        let should_clear;
        {
            let game_state = borrow_game_state(_self.game_state.as_ref().unwrap());
            should_clear = _self.game_in_looting_interaction(&game_state);
        }
        if should_clear {
            _self
                .ongoing_transfer_viz
                .as_mut()
                .unwrap()
                .bind_mut()
                .clear_transfer_ui();
        }
    }
}

impl LootViz {
    pub fn updated(&mut self) {
        self.base_mut().set_visible(true);

        let game_state_transfer;
        {
            let game_state = borrow_game_state(self.game_state.as_ref().unwrap());
            if let OngoingInteraction::Loot { transfer, .. } = game_state
                .unwrap_in_dungeon()
                .ongoing_event
                .as_ref()
                .unwrap()
                .unwrap_interaction()
            {
                game_state_transfer = transfer.clone();
            } else {
                panic!("Updating LootViz without an ongoing loot interaction");
            }
        }
        OngoingTransferViz::updated(
            self.ongoing_transfer_viz.as_mut().unwrap().clone(),
            &game_state_transfer,
        )
    }
}

#[godot_api]
impl IControl for LootViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            ongoing_transfer_viz: None,
            base,
        }
    }

    fn ready(&mut self) {
        let mut game_state: Gd<GameStateViz> = walk_parents_for(&self.to_gd());
        self.game_state = Some(game_state.clone());
        game_state.connect(
            GameStateViz::PRE_UPDATED_STATE_SIGNAL.into(),
            self.base().callable("pre_updated"),
        );

        let di_context = DiContext::get_nearest(self.base().clone().upcast()).unwrap();
        let di_context = di_context.bind();

        self.ongoing_transfer_viz =
            Some(di_context.get_registered_node_template::<OngoingTransferViz>("".into()));
        self.ongoing_transfer_viz
            .as_mut()
            .unwrap()
            .bind_mut()
            .set_inputs(Box::new(LootInput::transfer), Box::new(LootInput::finish));
    }
}

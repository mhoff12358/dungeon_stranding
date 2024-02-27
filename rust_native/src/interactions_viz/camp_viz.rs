use ds_lib::game_state::inputs::camp_input::CampInput;
use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext, game_state_viz::GameStateViz, tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct CampViz {
    amount: i32,

    game_state: Option<Gd<GameStateViz>>,

    amount_label: Option<Gd<Label>>,

    base: Base<Control>,
}

#[godot_api]
impl CampViz {
    fn get_game_state(this: &mut Gd<Self>) -> Gd<GameStateViz> {
        this.bind_mut().game_state.as_mut().unwrap().clone()
    }

    #[func(gd_self)]
    pub fn change_amount(mut this: Gd<Self>, delta: i32) {
        let amount = this.bind().amount;
        GameStateViz::accept_input(
            Self::get_game_state(&mut this),
            &CampInput::pick_amount(amount + delta),
        );
    }

    #[func(gd_self)]
    pub fn camp(mut this: Gd<Self>) {
        GameStateViz::accept_input(Self::get_game_state(&mut this), &CampInput::camp());
    }

    #[func(gd_self)]
    pub fn cancel(mut this: Gd<Self>) {
        GameStateViz::accept_input(Self::get_game_state(&mut this), &CampInput::cancel());
    }

    #[func]
    pub fn hide(&mut self) {
        self.to_gd().set_visible(false);
    }
}

impl CampViz {
    pub fn updated(&mut self, amount: &i32) {
        self.to_gd().set_visible(true);
        self.amount_label
            .as_mut()
            .unwrap()
            .set_text(format!("Amount: {}", *amount).into());
        self.amount = *amount;
    }
}

#[godot_api]
impl IControl for CampViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            amount: 0,
            game_state: None,
            amount_label: None,
            base,
        }
    }

    fn ready(&mut self) {
        let gd_self = self.to_gd();

        self.game_state = Some(walk_parents_for(&gd_self));
        self.game_state
            .as_mut()
            .unwrap()
            .connect("pre_updated_state".into(), gd_self.callable("hide"));

        let context = DiContext::get_nearest_bound(self.base().clone());
        self.amount_label = Some(context.get_registered_node_template("".into()));
    }
}

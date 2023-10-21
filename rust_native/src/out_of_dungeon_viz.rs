use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};

use crate::game_state_viz::{borrow_game_state, GameStateViz};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct OutOfDungeonViz {
    game_state: Option<Gd<GameStateViz>>,

    #[base]
    base: Base<Control>,
}

impl OutOfDungeonViz {
    pub const UPDATED_STATE_SIGNAL: &'static str = "updated_state";
}

#[godot_api]
impl OutOfDungeonViz {
    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().clone()
    }

    pub fn is_out_of_dungeon_impl(&self) -> bool {
        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        return game_state.is_out_of_dungeon();
    }

    #[func(gd_self)]
    pub fn is_out_of_dungeon(this: Gd<Self>) -> bool {
        let _self = this.bind();
        return _self.is_out_of_dungeon_impl();
    }

    #[func(gd_self)]
    pub fn _on_game_state_updated(mut this: Gd<OutOfDungeonViz>) {
        let is_out_of_dungeon;
        {
            let mut _self = this.bind_mut();
            is_out_of_dungeon = Self::is_out_of_dungeon_impl(&_self);
            _self.base.set_visible(is_out_of_dungeon);
        }
        if is_out_of_dungeon {
            this.emit_signal(Self::UPDATED_STATE_SIGNAL.into(), &[]);
        }
    }
}

#[godot_api]
impl ControlVirtual for OutOfDungeonViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(self.base.get_parent().unwrap().cast());
        self.game_state.as_mut().unwrap().connect(
            GameStateViz::UPDATED_STATE_SIGNAL.into(),
            self.base.callable("_on_game_state_updated"),
        );
    }
}

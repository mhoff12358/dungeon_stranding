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

#[godot_api]
impl OutOfDungeonViz {
    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().share()
    }

    #[func(gd_self)]
    pub fn is_out_of_dungeon(this: Gd<Self>) -> bool {
        let _self = this.bind();
        let game_state = borrow_game_state(&_self.game_state.as_ref().unwrap());
        return game_state.is_out_of_dungeon();
    }
    /*#[func]
    pub fn is_out_of_dungeon(&self) -> bool {
        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        return game_state.is_out_of_dungeon();
    }*/
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
    }
}

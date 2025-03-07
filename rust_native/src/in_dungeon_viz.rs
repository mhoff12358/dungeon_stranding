use std::ops::Deref;

use ds_lib::{
    dungeon_state::floor_state::FloorState,
    game_state::game_state::{GameState, InDungeon},
};
use godot::{
    engine::{Control, IControl},
    prelude::*,
};
use owning_ref::OwningHandle;

use crate::game_state_viz::{borrow_game_state, GameStateViz};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InDungeonViz {
    game_state: Option<Gd<GameStateViz>>,

    #[export]
    map_controlable: bool,

    base: Base<Control>,
}

impl InDungeonViz {
    pub const UPDATED_STATE_SIGNAL: &'static str = "updated_state";
    pub const UPDATED_STATE_FIGHT_SIGNAL: &'static str = "updated_state_fight";
    pub const UPDATED_STATE_INTERACTION_SIGNAL: &'static str = "updated_state_interaction";
}

#[godot_api]
impl InDungeonViz {
    #[signal]
    pub fn updated_state();

    #[signal]
    pub fn updated_state_fight();

    #[signal]
    pub fn updated_state_interaction();

    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().clone()
    }

    #[func]
    pub fn is_in_dungeon(&self) -> bool {
        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        return game_state.is_in_dungeon();
    }

    #[func]
    pub fn is_moving_in_dungeon(&self) -> bool {
        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        match game_state.deref() {
            GameState::InDungeon(in_dungeon) => in_dungeon.ongoing_event.is_none(),
            _ => false,
        }
    }

    #[func]
    pub fn get_tiles(&self) -> GString {
        let floor = self.borrow_current_floor();
        let stairs = floor.layout().stairs();
        return format!("{}, {}", stairs.up.x, stairs.up.y).into();
    }

    #[func(gd_self)]
    pub fn _on_game_state_updated(mut this: Gd<InDungeonViz>) {
        let is_in_dungeon;
        let mut is_fight = false;
        let mut is_interaction = false;
        {
            let mut _self = this.bind_mut();
            is_in_dungeon = _self.is_in_dungeon();
            _self.base_mut().set_visible(is_in_dungeon);
            if is_in_dungeon {
                let in_dungeon = _self.borrow_in_dungeon();
                if let Some(event) = in_dungeon.ongoing_event.as_ref() {
                    is_fight = event.is_fight();
                    is_interaction = event.is_interaction();
                }
            }
            _self.map_controlable = is_in_dungeon && !is_fight && !is_interaction;
        }
        if is_in_dungeon {
            this.emit_signal(Self::UPDATED_STATE_SIGNAL.into(), &[]);
            if is_fight {
                this.emit_signal(Self::UPDATED_STATE_FIGHT_SIGNAL.into(), &[]);
            }
            if is_interaction {
                this.emit_signal(Self::UPDATED_STATE_INTERACTION_SIGNAL.into(), &[]);
            }
        }
    }
}

impl InDungeonViz {
    pub fn borrow_in_dungeon<'a>(&'a self) -> impl Deref<Target = InDungeon> + 'a {
        OwningHandle::new_with_fn(
            borrow_game_state(&self.game_state.as_ref().unwrap()),
            |it: *const GameState| {
                let it = unsafe { &*it };
                it.unwrap_in_dungeon()
            },
        )
    }

    pub fn borrow_current_floor<'a>(&'a self) -> impl Deref<Target = FloorState> + 'a {
        OwningHandle::new_with_fn(
            borrow_game_state(&self.game_state.as_ref().unwrap()),
            |it: *const GameState| {
                let it = unsafe { &*it };
                it.unwrap_in_dungeon().get_current_floor()
            },
        )
    }
}

#[godot_api]
impl IControl for InDungeonViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            map_controlable: false,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(self.to_gd().get_parent().unwrap().cast());

        let _on_game_state_updated = self.base().callable("_on_game_state_updated");
        self.game_state.as_mut().unwrap().connect(
            GameStateViz::UPDATED_STATE_SIGNAL.into(),
            _on_game_state_updated,
        );
    }
}

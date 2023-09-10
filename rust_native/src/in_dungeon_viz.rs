use std::ops::Deref;

use ds_lib::{
    dungeon_state::floor_layout::FloorLayout,
    game_state::game_state::{GameState, InDungeon},
};
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};
use owning_ref::OwningHandle;

use crate::game_state_viz::{borrow_game_state, GameStateViz};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct InDungeonViz {
    game_state: Option<Gd<GameStateViz>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl InDungeonViz {
    #[signal]
    pub fn updated_state();

    #[func]
    pub fn game_state(&self) -> Gd<GameStateViz> {
        self.game_state.as_ref().unwrap().share()
    }

    #[func]
    pub fn is_in_dungeon(&self) -> bool {
        let game_state = borrow_game_state(&self.game_state.as_ref().unwrap());
        return game_state.is_in_dungeon();
    }

    #[func]
    pub fn get_tiles(&self) -> GodotString {
        let floor = self.borrow_current_floor();
        let stairs = floor.stairs();
        return format!("{}, {}", stairs.up.x, stairs.up.y).into();
    }

    #[func(gd_self)]
    pub fn _on_game_state_updated(mut this: Gd<InDungeonViz>) {
        let is_in_dungeon;
        {
            println!("A");
            let mut _self = this.bind_mut();
            println!("B");
            is_in_dungeon = _self.is_in_dungeon();
            println!("C");
            _self.base.set_visible(is_in_dungeon);
            println!("D");
        }
        if is_in_dungeon {
            println!("E");
            this.emit_signal("updated_state".into(), &[]);
            println!("F");
        }
    }
    /*pub fn _on_game_state_updated(&mut self) {}
    pub fn _on_game_state_updated(&mut self) {
        let is_in_dungeon = self.is_in_dungeon();
        self.base.set_visible(is_in_dungeon);
        if is_in_dungeon {
            self.base.emit_signal("updated_state".into(), &[]);
        }
    }*/
    //pub fn _on_game_state_updated(mut this: Gd<InDungeonViz>) {
    /*
    let is_in_dungeon;
    {
        println!("A");
        let mut _self = this.bind_mut();
        println!("B");
        is_in_dungeon = _self.is_in_dungeon();
        println!("C");
        _self.base.set_visible(is_in_dungeon);
        println!("D");
    }
    if is_in_dungeon {
        println!("E");
        this.emit_signal("updated_state".into(), &[]);
        println!("F");
    }
    println!("G");
    */
    //}
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

    pub fn borrow_current_floor<'a>(&'a self) -> impl Deref<Target = FloorLayout> + 'a {
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
impl ControlVirtual for InDungeonViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            base,
        }
    }

    fn enter_tree(&mut self) {
        self.game_state = Some(self.base.get_parent().unwrap().cast());
        self.game_state.as_mut().unwrap().connect(
            "updated_state".into(),
            self.base.callable("_on_game_state_updated"),
        );
    }
}

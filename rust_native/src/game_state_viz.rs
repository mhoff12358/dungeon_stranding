use std::cell::Ref;
use std::ops::{Deref, DerefMut};

use crate::tree_utils::walk_parents_for;
use crate::{app::App, my_gd_ref::MyGdRef};
use ds_lib::directions::Direction;
use ds_lib::game_state::game_state::GameState;
use ds_lib::game_state::game_state_input::accept_game_state_input;
use ds_lib::game_state::inputs::game_state_input::GameStateInput;
use ds_lib::input::keycode::KeyCode;
use godot::engine::{Control, ControlVirtual};
use godot::prelude::*;
use owning_ref::{OwningHandle, StableAddress};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameStateViz {
    app: Option<Gd<App>>,

    #[base]
    base: Base<Control>,
}

enum GodotInput {
    Quit,
    Direction(Direction),
    Number(u8),
    Select,
    Cancel,
}

impl GameStateViz {
    pub const PRE_UPDATED_STATE_SIGNAL: &str = "pre_updated_state";
    pub const UPDATED_STATE_SIGNAL: &str = "updated_state";
}

#[godot_api]
impl GameStateViz {
    #[signal]
    fn pre_updated_state();

    #[signal]
    fn updated_state();

    #[func(gd_self)]
    fn handle_input(mut this: Gd<GameStateViz>, input: i32) {
        let gd_input = match input {
            0 => GodotInput::Quit,
            1 => GodotInput::Number(1),
            2 => GodotInput::Number(2),
            3 => GodotInput::Number(3),
            4 => GodotInput::Number(4),
            5 => GodotInput::Number(5),
            6 => GodotInput::Number(6),
            7 => GodotInput::Number(7),
            8 => GodotInput::Number(8),
            9 => GodotInput::Number(9),
            10 => GodotInput::Number(10),
            11 => GodotInput::Direction(Direction::Right),
            12 => GodotInput::Direction(Direction::Up),
            13 => GodotInput::Direction(Direction::Left),
            14 => GodotInput::Direction(Direction::Down),
            15 => GodotInput::Select,
            16 => GodotInput::Cancel,

            _ => {
                panic!("Invalid input");
            }
        };

        let key_code = match gd_input {
            GodotInput::Direction(Direction::Right) => KeyCode::Char('d'),
            GodotInput::Direction(Direction::Up) => KeyCode::Char('w'),
            GodotInput::Direction(Direction::Left) => KeyCode::Char('a'),
            GodotInput::Direction(Direction::Down) => KeyCode::Char('s'),
            GodotInput::Number(i) => match i {
                1..=10 => KeyCode::Char(('1' as u8 + (i - 1)) as char),
                _ => {
                    panic!("Invalid number somehow");
                }
            },
            GodotInput::Select => KeyCode::Char(' '),
            GodotInput::Cancel => KeyCode::Char('q'),
            GodotInput::Quit => KeyCode::Esc,
        };

        {
            let mut this = this.bind_mut();
            let mut app_bind = this.app.as_mut().unwrap().bind_mut();
            let app = app_bind.get_app_mut();
            ds_lib::handle_keypress(key_code, app);
            ds_lib::game_state::state_updates::update_algos::check_invariants(app);
        }
        this.emit_signal(Self::PRE_UPDATED_STATE_SIGNAL.into(), &[]);
        this.emit_signal(Self::UPDATED_STATE_SIGNAL.into(), &[]);
    }

    #[func(gd_self)]
    pub fn handle_game_update(mut this: Gd<Self>) {
        {
            let mut this = this.bind_mut();
            let mut app_bind = this.app.as_mut().unwrap().bind_mut();
            let app = app_bind.get_app_mut();
            ds_lib::game_state::state_updates::update_algos::check_invariants(app);
        }
        this.emit_signal(Self::PRE_UPDATED_STATE_SIGNAL.into(), &[]);
        this.emit_signal(Self::UPDATED_STATE_SIGNAL.into(), &[]);
    }
}

impl GameStateViz {
    pub fn borrow_game_state<'a>(&'a self) -> impl Deref<Target = GameState> + 'a {
        OwningHandle::new_with_fn(
            MyGdRef::new(self.app.as_ref().unwrap().bind()),
            |it: *const App| -> Ref<'a, GameState> {
                let it = unsafe { &*it };
                it.get_app().game_state.borrow()
            },
        )
    }

    pub fn accept_input(this: Gd<Self>, input: &GameStateInput) {
        {
            let _self = this.bind();
            let app = _self.app.as_ref().unwrap().bind();
            let game_state = app.get_app().game_state.deref();
            accept_game_state_input(game_state, input);
        }
        Self::handle_game_update(this);
    }

    pub fn accept_input_from_child(indirect_child: &Gd<Node>, input: &GameStateInput) {
        Self::accept_input(Self::get_game_state_from_node(indirect_child), input);
    }

    fn get_game_state_from_node(this: &Gd<Node>) -> Gd<GameStateViz> {
        walk_parents_for(&this)
    }
}

pub fn borrow_game_state<'a>(
    game_state: &'a Gd<GameStateViz>,
) -> impl Deref<Target = GameState> + StableAddress + 'a {
    fn internal_borrow<'b>(
        it: *const GameStateViz,
    ) -> impl Deref<Target = GameState> + StableAddress + 'b {
        fn internal_borrow<'c>(
            it: *const App,
        ) -> impl Deref<Target = GameState> + StableAddress + 'c {
            let it = unsafe { &*it };
            it.get_app().game_state.borrow()
        }
        let it = unsafe { &*it };
        OwningHandle::new_with_fn(
            MyGdRef::new(it.app.as_ref().unwrap().bind()),
            &internal_borrow,
        )
    }
    OwningHandle::new_with_fn(MyGdRef::new(game_state.bind()), &internal_borrow)
}

pub fn borrow_game_state_mut<'a>(
    game_state: &'a Gd<GameStateViz>,
) -> impl DerefMut<Target = GameState> + StableAddress + 'a {
    fn internal_borrow<'b>(
        it: *const GameStateViz,
    ) -> impl DerefMut<Target = GameState> + StableAddress + 'b {
        fn internal_borrow<'c>(
            it: *const App,
        ) -> impl DerefMut<Target = GameState> + StableAddress + 'c {
            let it = unsafe { &*it };
            it.get_app().game_state.borrow_mut()
        }
        let it = unsafe { &*it };
        OwningHandle::new_with_fn(
            MyGdRef::new(it.app.as_ref().unwrap().bind()),
            &internal_borrow,
        )
    }
    OwningHandle::new_with_fn(MyGdRef::new(game_state.bind()), &internal_borrow)
}

#[godot_api]
impl ControlVirtual for GameStateViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self { base, app: None }
    }

    fn ready(&mut self) {
        self.app = Some(App::find_app(&self.base.clone().upcast()));
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

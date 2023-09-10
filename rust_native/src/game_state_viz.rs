use std::cell::Ref;
use std::ops::Deref;

use crate::{app::App, my_gd_ref::MyGdRef};
use ds_lib::directions::Direction;
use ds_lib::game_state::game_state::GameState;
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
}

#[godot_api]
impl GameStateViz {
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
            GodotInput::Quit => KeyCode::Esc,
        };

        {
            let mut this = this.bind_mut();
            let mut app_bind = this.app.as_mut().unwrap().bind_mut();
            let app = app_bind.get_app_mut();
            ds_lib::handle_keypress(key_code, app);
            ds_lib::game_state::state_updates::update_algos::check_invariants(app);
        }
        this.emit_signal("updated_state".into(), &[]);
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

#[godot_api]
impl ControlVirtual for GameStateViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self { base, app: None }
    }

    fn ready(&mut self) {
        self.app = Some(App::find_app(&self.base.share().upcast()));

        //let mut base = self.base.share();
        //drop(self);
        //self.base.emit_signal("updated_state".into(), &[]);
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

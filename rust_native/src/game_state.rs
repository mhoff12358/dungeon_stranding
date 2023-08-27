use crate::app::App;
use ds_lib::directions::Direction;
use ds_lib::handle_keypress;
use ds_lib::input::keycode::KeyCode;
use godot::engine::{Control, ControlVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameState {
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
impl GameState {
    #[func]
    fn handle_input(&mut self, input: i32) {
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
            GodotInput::Direction(Direction::Right) => KeyCode::Char('D'),
            GodotInput::Direction(Direction::Up) => KeyCode::Char('W'),
            GodotInput::Direction(Direction::Left) => KeyCode::Char('A'),
            GodotInput::Direction(Direction::Down) => KeyCode::Char('S'),
            GodotInput::Number(i) => match i {
                1..=10 => KeyCode::Char(('1' as u8 + (i - 1)) as char),
                _ => {
                    panic!("Invalid number somehow");
                }
            },
            GodotInput::Select => KeyCode::Char(' '),
            GodotInput::Quit => KeyCode::Esc,
        };

        let mut app_bind = self.app.as_mut().unwrap().bind_mut();
        let app = app_bind.get_app_mut();
        handle_keypress(key_code, app);
    }
}

#[godot_api]
impl ControlVirtual for GameState {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        gd_log!("Creating GameState");

        Self { base, app: None }
    }

    fn ready(&mut self) {
        self.app = Some(App::find_app(&self.base.share().upcast()));
        gd_log!(
            "GameState found app: {}",
            self.app.as_ref().unwrap().bind().get_name()
        );
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

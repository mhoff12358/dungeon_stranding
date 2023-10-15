use ds_lib::{directions::Direction, game_state::inputs::dig_input::DigInput};
use godot::{
    engine::{Control, ControlVirtual},
    prelude::*,
};

use crate::{game_state_viz::GameStateViz, tree_utils::walk_parents_for};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct DigViz {
    game_state: Option<Gd<GameStateViz>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl DigViz {
    #[func(gd_self)]
    pub fn cancel(mut this: Gd<Self>) {
        GameStateViz::accept_input(Self::get_game_state(&mut this), &DigInput::cancel());
    }

    #[func(gd_self)]
    pub fn dig_left(this: Gd<Self>) {
        Self::dig_direction(this, Direction::Left);
    }

    #[func(gd_self)]
    pub fn dig_right(this: Gd<Self>) {
        Self::dig_direction(this, Direction::Right);
    }

    #[func(gd_self)]
    pub fn dig_up(this: Gd<Self>) {
        Self::dig_direction(this, Direction::Up);
    }

    #[func(gd_self)]
    pub fn dig_down(this: Gd<Self>) {
        Self::dig_direction(this, Direction::Down);
    }

    pub fn dig_direction(mut this: Gd<Self>, direction: Direction) {
        GameStateViz::accept_input(
            Self::get_game_state(&mut this),
            &DigInput::do_dig(direction),
        );
    }

    #[func]
    pub fn hide(&mut self) {
        self.base.set_visible(false);
    }
}

impl DigViz {
    pub fn updated(&mut self) {
        self.base.set_visible(true);
    }

    fn get_game_state(this: &mut Gd<Self>) -> Gd<GameStateViz> {
        this.bind_mut().game_state.as_mut().unwrap().clone()
    }
}

#[godot_api]
impl ControlVirtual for DigViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.game_state = Some(walk_parents_for(&self.base));
        self.game_state
            .as_mut()
            .unwrap()
            .connect("pre_updated_state".into(), self.base.callable("hide"));
    }

    /*fn gui_input(&mut self, event: Gd<InputEvent>) {
        if let Some(keyboard_event) = event.try_cast::<InputEventKey>() {
            match keyboard_event.get_keycode() {
                Key::KEY_W => self.dig_up(),
                Key::KEY_S => self.dig_down(),
                Key::KEY_A => self.dig_left(),
                Key::KEY_D => self.dig_right(),
                _ => {}
            }
        }
    }*/
}

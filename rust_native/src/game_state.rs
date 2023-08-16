use std::ops::Deref;

use crate::app::App;
use godot::engine::{Control, ControlVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameState {
    app: Option<Gd<App>>,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl ControlVirtual for GameState {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        godot::engine::utilities::print("Creating GameState".to_variant(), &[]);

        Self { base, app: None }
    }

    fn ready(&mut self) {
        self.app = Some(App::find_app(&self.base.share().upcast()));
        godot::engine::utilities::print(
            "GameState found app: ".to_variant(),
            &[self.app.as_ref().unwrap().bind().get_name().to_variant()],
        );
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

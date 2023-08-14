use godot::engine::{Control, ControlVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Control)]
struct GameState {
    #[base]
    base: Base<Control>,
}

#[godot_api]
impl ControlVirtual for GameState {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        godot::engine::utilities::print("Creating GameState".to_variant(), &[]);

        Self { base }
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

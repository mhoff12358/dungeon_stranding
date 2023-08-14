use ds_lib::cli_args::CliArgs;
use godot::engine::{Node, NodeVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
struct App {
    app: ds_lib::app::App,

    #[base]
    base: Base<Node>,
}

#[godot_api]
impl NodeVirtual for App {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        godot::engine::utilities::print("Creating App".to_variant(), &[]);

        let args = CliArgs::default();

        Self {
            app: ds_lib::app::App::new(&args),
            base,
        }
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

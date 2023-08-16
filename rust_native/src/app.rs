use ds_lib::cli_args::CliArgs;
use godot::engine::{Node, NodeVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct App {
    app: ds_lib::app::App,

    name: GodotString,

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
            name: "AN APP!".into(),
            base,
        }
    }

    fn to_string(&self) -> godot::builtin::GodotString {
        self.base.to_string().into()
    }
}

impl App {
    pub fn find_app(node: &Gd<Node>) -> Gd<App> {
        let tree = node.get_tree().unwrap();
        let scene_tree = tree.get_current_scene().unwrap();
        return look_for_app(&scene_tree).unwrap();
    }

    pub fn get_name(&self) -> &GodotString {
        return &self.name;
    }
}

fn look_for_app(node: &Gd<Node>) -> Option<Gd<App>> {
    if node.is_class(App::class_name().to_godot_string()) {
        return Some(node.share().cast());
    }
    for child_index in 0..node.get_child_count() {
        if let Some(look_children) = look_for_app(node.get_child(child_index).as_ref().unwrap()) {
            return Some(look_children);
        }
    }
    return None;
}

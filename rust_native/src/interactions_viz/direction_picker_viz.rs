use std::collections::HashSet;

use ds_lib::{
    directions::{Directed, Direction, DIRECTIONS},
    game_state::inputs::game_state_input::GameStateInput,
};
use godot::{
    engine::{Button, Control, IControl, Label},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext, game_state_viz::GameStateViz, tree_utils::walk_parents_for,
};

struct GodotObjects {
    label: Gd<Label>,
    cancel: Gd<Button>,
    directed: Directed<Gd<Button>>,
}

pub struct DirectionPickerConfig {
    pub cancel_input: Option<GameStateInput>,
    pub directed_input: Box<dyn Fn(Direction) -> GameStateInput>,
    pub allowed_directions: HashSet<Direction>,
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct DirectionPickerViz {
    game_state: Option<Gd<GameStateViz>>,

    objects: Option<GodotObjects>,

    config: Option<DirectionPickerConfig>,

    base: Base<Control>,
}

#[godot_api]
impl DirectionPickerViz {
    #[func(gd_self)]
    pub fn cancel(mut this: Gd<Self>) {
        let cancel_input;
        {
            let _self = this.bind_mut();
            cancel_input = _self.config.as_ref().unwrap().cancel_input.clone();
        }
        if let Some(cancel_input) = cancel_input {
            GameStateViz::accept_input(Self::get_game_state(&mut this), &cancel_input);
        }
    }

    #[func(gd_self)]
    pub fn pick_left(this: Gd<Self>) {
        Self::pick_direction(this, Direction::Left);
    }

    #[func(gd_self)]
    pub fn pick_right(this: Gd<Self>) {
        Self::pick_direction(this, Direction::Right);
    }

    #[func(gd_self)]
    pub fn pick_up(this: Gd<Self>) {
        Self::pick_direction(this, Direction::Up);
    }

    #[func(gd_self)]
    pub fn pick_down(this: Gd<Self>) {
        Self::pick_direction(this, Direction::Down);
    }

    pub fn pick_direction(mut this: Gd<Self>, direction: Direction) {
        let input;
        {
            let _self = this.bind_mut();
            input = (*_self.config.as_ref().unwrap().directed_input)(direction);
        }
        GameStateViz::accept_input(Self::get_game_state(&mut this), &input);
    }

    #[func]
    pub fn hide(&mut self) {
        self.to_gd().set_visible(false);
    }
}

impl DirectionPickerViz {
    pub fn updated(&mut self, display_text: String, config: DirectionPickerConfig) {
        self.to_gd().set_visible(true);
        self.config = Some(config);

        let objects = self.objects.as_mut().unwrap();
        objects.label.set_text(display_text.into());
        objects
            .cancel
            .set_visible(self.config.as_ref().unwrap().cancel_input.is_some());
        for direction in DIRECTIONS.iter() {
            objects.directed.get_mut(*direction).set_visible(
                self.config
                    .as_ref()
                    .unwrap()
                    .allowed_directions
                    .contains(direction),
            );
        }
    }

    fn get_game_state(this: &mut Gd<Self>) -> Gd<GameStateViz> {
        this.bind_mut().game_state.as_mut().unwrap().clone()
    }
}

#[godot_api]
impl IControl for DirectionPickerViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            objects: None,
            config: None,
            base,
        }
    }

    fn ready(&mut self) {
        let gd_self = self.to_gd();

        self.game_state = Some(walk_parents_for(&gd_self));
        self.game_state
            .as_mut()
            .unwrap()
            .connect("pre_updated_state".into(), gd_self.callable("hide"));

        let context = DiContext::get_nearest_bound(self.base().clone());
        self.objects = Some(GodotObjects {
            label: context.get_registered_node_template("".into()),
            cancel: context.get_registered_node_template("cancel".into()),
            directed: Directed::new([
                context.get_registered_node_template("right".into()),
                context.get_registered_node_template("up".into()),
                context.get_registered_node_template("left".into()),
                context.get_registered_node_template("down".into()),
            ]),
        });

        self.objects.as_mut().unwrap().cancel.connect(
            "pressed".into(),
            Callable::from_object_method(&gd_self, "cancel"),
        );

        let directed_callbacks = Directed::new(["pick_right", "pick_up", "pick_left", "pick_down"]);
        for direction in DIRECTIONS.iter() {
            let button = self.objects.as_mut().unwrap().directed.get_mut(*direction);
            button.connect(
                "pressed".into(),
                Callable::from_object_method(&gd_self, directed_callbacks.get_ref(*direction)),
            );
        }
    }
}

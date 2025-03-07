use ds_lib::game_state::{
    inputs::in_dungeon_input::InDungeonInput, state_updates::interactions::Interaction,
};
use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};

use crate::{
    game_state_viz::GameStateViz,
    in_dungeon_viz::InDungeonViz,
    template_spawners::{
        template_spawner::{TemplateControl, TemplateGenerics, TemplateSpawner},
        update_behavior::UpdateSpawnedTemplate,
    },
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AvailableInteractionViz {
    interaction: Option<Interaction>,

    #[export]
    label: Option<Gd<Label>>,

    base: Base<Control>,
}

impl TemplateControl for AvailableInteractionViz {
    type Value = Interaction;
    type Context = ();

    fn instantiate_template(&mut self, value: &Self::Value, _context: &Self::Context) {
        self.interaction = Some(value.clone());
        self.label
            .as_mut()
            .unwrap()
            .set_text(format!("{}", value.description()).into());
    }
}

#[godot_api]
impl AvailableInteractionViz {
    #[func(gd_self)]
    pub fn do_interaction(this: Gd<Self>) {
        let interaction = this.bind().interaction.as_ref().unwrap().clone();
        GameStateViz::accept_input_from_child(
            &this.upcast(),
            &InDungeonInput::do_interaction(interaction),
        );
    }
}

#[godot_api]
impl IControl for AvailableInteractionViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            interaction: None,
            label: None,
            base,
        }
    }
}

struct InteractionGenerics {}

impl TemplateGenerics for InteractionGenerics {
    type Key = Interaction;
    type Value = Interaction;
    type Context = ();
    type TemplateType = AvailableInteractionViz;
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AvailableInteractionsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    interaction_template: Option<Gd<AvailableInteractionViz>>,
    interactions_spawner:
        Option<TemplateSpawner<InteractionGenerics, UpdateSpawnedTemplate<InteractionGenerics>>>,
    base: Base<Control>,
}

#[godot_api]
impl AvailableInteractionsViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {
        self.base_mut().set_visible(true);
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let spawner = self.interactions_spawner.as_mut().unwrap();
        spawner.update_ref(in_dungeon.interactions.iter(), &());
    }
}

#[godot_api]
impl IControl for AvailableInteractionsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,

            interaction_template: None,
            interactions_spawner: None,

            base,
        }
    }

    fn ready(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.to_gd()));
        let _on_in_dungeon_updated = self.base().callable("_on_in_dungeon_updated");
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            _on_in_dungeon_updated,
        );

        self.interactions_spawner = Some(TemplateSpawner::new(
            self.interaction_template.as_ref().unwrap().clone(),
        ));
    }
}

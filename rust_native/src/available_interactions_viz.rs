use ds_lib::game_state::state_updates::interactions::Interaction;
use godot::{
    engine::{Control, ControlVirtual, Label},
    prelude::*,
};
use num_traits::cast::ToPrimitive;

use crate::{
    in_dungeon_viz::InDungeonViz,
    template_spawner::{Template, TemplateControl, TemplateSpawner},
    tree_utils::walk_parents_for,
};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AvailableInteractionViz {
    interaction: Option<Interaction>,

    #[export]
    label: Option<Gd<Label>>,

    #[base]
    base: Base<Control>,
}

impl TemplateControl for AvailableInteractionViz {
    type Value = Interaction;

    fn instantiate_template(&mut self, value: &Self::Value) {
        self.label
            .as_mut()
            .unwrap()
            .set_text(format!("{:?}", value).into());
    }

    fn control(&self) -> &Self::Base {
        &self.base
    }

    fn control_mut(&mut self) -> &mut Self::Base {
        &mut self.base
    }
}

#[godot_api]
impl AvailableInteractionViz {}

#[godot_api]
impl ControlVirtual for AvailableInteractionViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            interaction: None,
            label: None,
            base,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct AvailableInteractionsViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    interaction_template: Option<Gd<AvailableInteractionViz>>,
    interactions_spawner:
        Option<TemplateSpawner<Interaction, Interaction, AvailableInteractionViz>>,
    #[base]
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
        self.base.set_visible(true);
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let spawner = self.interactions_spawner.as_mut().unwrap();
        spawner.update(
            in_dungeon.interactions.iter().map(|inter| *inter),
            |interaction| *interaction,
        );
    }
}

impl AvailableInteractionsViz {
    fn get_interactions(interactions: &Vec<Interaction>) -> GodotString {
        let mut text = String::new();
        for (index, interaction) in interactions.iter().enumerate() {
            text = format!("{}{}: {}\n", text, index + 1, interaction.description());
        }
        return text.into();
    }
}

#[godot_api]
impl ControlVirtual for AvailableInteractionsViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,

            interaction_template: None,
            interactions_spawner: None,

            base,
        }
    }

    fn ready(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            "updated_state".into(),
            self.base.callable("_on_in_dungeon_updated"),
        );

        self.interactions_spawner = Some(TemplateSpawner::new(
            self.interaction_template.as_ref().unwrap().clone(),
        ));
    }
}

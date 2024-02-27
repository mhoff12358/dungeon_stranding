use ds_lib::{
    dungeon_state::encounters::wandering_encounters::WanderingEncounterId,
    game_state::{inputs::scout_input::ScoutInput, state_updates::scout_results::ScoutResults},
};
use godot::{
    engine::{Control, IControl, Label},
    prelude::*,
};

use crate::{
    di_context::di_context::DiContext,
    game_state_viz::GameStateViz,
    template_spawners::{
        template_spawner::{TemplateGenerics, TemplateSpawner},
        update_behavior::TemplateSpawnerUpdateBehavior,
    },
    tree_utils::walk_parents_for,
};

make_id_type!(WanderingEncounterId);

pub struct ScoutedEncounterGenerics {}

impl TemplateGenerics for ScoutedEncounterGenerics {
    type Key = WanderingEncounterIdGodot;
    type Value = WanderingEncounterIdGodot;
    type Context = ScoutResults;
    type TemplateType = Control;
}

#[derive(GodotClass)]
#[class(base=Control)]
pub struct ScoutViz {
    game_state: Option<Gd<GameStateViz>>,

    spawner: Option<TemplateSpawner<ScoutedEncounterGenerics, ScoutViz>>,

    base: Base<Control>,
}

impl TemplateSpawnerUpdateBehavior for ScoutViz {
    type Generics = ScoutedEncounterGenerics;

    fn initialize(
        mut _template: Gd<Control>,
        _value: &WanderingEncounterIdGodot,
        _context: &ScoutResults,
    ) {
    }

    fn update_template(
        template: Gd<Control>,
        value: &WanderingEncounterIdGodot,
        context: &ScoutResults,
        _previous: &Option<Gd<<Self::Generics as TemplateGenerics>::TemplateType>>,
    ) {
        let di_context = DiContext::get_nearest_bound(template.clone());
        let mut id: Gd<Label> = di_context.get_registered_node_template("id".into());
        let mut name: Gd<Label> = di_context.get_registered_node_template("name".into());
        let mut distance: Gd<Label> = di_context.get_registered_node_template("distance".into());

        if let Some(scouted_encounter) = context.get_encounter(value.0) {
            let encounter = scouted_encounter.encounter.borrow();
            id.set_text(format!("{}", encounter.id.0).into());
            name.set_text(format!("{}", encounter.monster.name).into());
            distance.set_text(format!("{}", scouted_encounter.distance).into());
        } else {
            id.set_text("0".into());
            name.set_text("ERROR".into());
            distance.set_text("0".into());
        }
    }
}

#[godot_api]
impl ScoutViz {
    #[func(gd_self)]
    pub fn cancel(mut this: Gd<Self>) {
        this.bind_mut().updated(&ScoutResults::new());
        GameStateViz::accept_input(Self::get_game_state(&mut this), &ScoutInput::cancel());
    }

    #[func]
    pub fn hide(&mut self) {
        self.to_gd().set_visible(false);
    }
}

impl ScoutViz {
    fn get_game_state(this: &mut Gd<Self>) -> Gd<GameStateViz> {
        this.bind_mut().game_state.as_mut().unwrap().clone()
    }

    pub fn updated(&mut self, results: &ScoutResults) {
        self.to_gd().set_visible(true);
        self.spawner.as_mut().unwrap().update(
            results
                .encounters
                .iter()
                .map(|encounter| encounter.encounter.borrow().id.into()),
            results,
        );
    }
}

#[godot_api]
impl IControl for ScoutViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            game_state: None,
            spawner: None,
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
        let template: Gd<Control> = context.get_registered_node_template("template".into());
        self.spawner = Some(TemplateSpawner::new(template));
    }
}

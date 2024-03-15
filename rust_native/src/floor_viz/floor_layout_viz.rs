use std::ops::Deref;

use ds_lib::{
    coord::Coord,
    dungeon_state::{
        tile_entity::TileEntity,
        tile_state::{OpenType, SpecificTS, TileState},
    },
    game_state::game_state::InDungeon,
};
use godot::{
    engine::{Control, IControl, TileMap},
    prelude::*,
};
use smallvec::{smallvec, SmallVec};

use crate::{
    di_context::di_context::DiContext, in_dungeon_viz::InDungeonViz, tree_utils::walk_parents_for,
};

use super::tile_spacing::TileSpacing;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct FloorLayoutViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    tile_map: Option<Gd<TileMap>>,
    #[export]
    player: Option<Gd<Node2D>>,
    #[export]
    entities: Option<Gd<Node2D>>,

    #[export]
    wall_atlas_coord: Vector2i,
    #[export]
    hallway_atlas_coord: Vector2i,
    #[export]
    room_atlas_coord: Vector2i,
    #[export]
    closed_door_atlas_coord: Vector2i,
    #[export]
    open_door_atlas_coord: Vector2i,
    #[export]
    door_pitoned_atlas_coord: Vector2i,
    #[export]
    stairs_up_atlas_coord: Vector2i,
    #[export]
    stairs_down_atlas_coord: Vector2i,

    #[export]
    body_entity_scene_path: GString,
    body_entity_scene: Option<Gd<PackedScene>>,
    #[export]
    campfire_entity_scene_path: GString,
    campfire_entity_scene: Option<Gd<PackedScene>>,
    #[export]
    gold_entity_scene_path: GString,
    gold_entity_scene: Option<Gd<PackedScene>>,
    #[export]
    chest_entity_scene_path: GString,
    chest_entity_scene: Option<Gd<PackedScene>>,

    base: Base<Control>,
}

#[godot_api]
impl FloorLayoutViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().clone()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let floor = in_dungeon.get_current_floor();
        let bounds = floor.layout().bounds();
        for y in bounds.min.y..=bounds.max.y {
            for x in bounds.min.x..=bounds.max.x {
                let tile_coord = Coord { x, y };
                let atlas_coords = self.get_atlas_coords_for_tile(
                    &in_dungeon,
                    &tile_coord,
                    floor.layout().tiles().get(&tile_coord),
                );
                let tile_map = self.tile_map.as_mut().unwrap();
                for layer in 0..TILE_LAYERS {
                    if layer < atlas_coords.len() {
                        tile_map
                            .set_cell_ex(layer as i32, Vector2i { x, y })
                            .source_id(0)
                            .atlas_coords(atlas_coords[layer])
                            .done();
                    } else {
                        tile_map
                            .set_cell_ex(layer as i32, Vector2i { x, y })
                            .source_id(-1)
                            .done();
                    }
                }
                for (layer, atlas_coord) in atlas_coords.into_iter().enumerate() {
                    tile_map
                        .set_cell_ex(layer as i32, Vector2i { x, y })
                        .source_id(0)
                        .atlas_coords(atlas_coord)
                        .done();
                }
            }
        }

        let tile_spacing = TileSpacing::new(self.tile_map.as_ref().unwrap());

        let entities = self.entities.as_mut().unwrap();
        let entities_children = entities.get_children();
        for i in 0..entities_children.len() {
            let mut child = entities_children.get(i);
            entities.remove_child(child.clone());
            child.queue_free();
        }
        for (coord, entity) in floor.entities().all_entities_iter() {
            let entity = entity.borrow();
            let entity_scene = match entity.deref() {
                TileEntity::DeadBody(_) => self.body_entity_scene.as_ref().unwrap(),
                TileEntity::GoldPile(_) => self.gold_entity_scene.as_ref().unwrap(),
                TileEntity::CampSite => self.campfire_entity_scene.as_ref().unwrap(),
                TileEntity::Chest(_) => self.chest_entity_scene.as_ref().unwrap(),
            };
            let entity = entity_scene.instantiate().unwrap();
            let mut entity2d: Gd<Node2D> = entity.clone().cast();
            entities.add_child(entity);
            entity2d.set_position(tile_spacing.entity_position(coord));
        }

        let player = self.player.as_mut().unwrap();
        player.set_position(tile_spacing.entity_position(in_dungeon.player_position));
    }
}
const TILE_LAYERS: usize = 4;
type AtlasCoordsForTile = SmallVec<[Vector2i; TILE_LAYERS]>;

impl FloorLayoutViz {
    fn get_atlas_coords_for_tile(
        &self,
        in_dungeon: &InDungeon,
        coord: &Coord,
        tile: Option<&TileState>,
    ) -> AtlasCoordsForTile {
        if tile.is_none() {
            return smallvec![-Vector2i::ONE];
        }

        let floor_layout = in_dungeon.get_current_floor().layout();

        if floor_layout.stairs().up == *coord {
            return smallvec![self.stairs_up_atlas_coord];
        } else if floor_layout.stairs().down == *coord {
            return smallvec![self.stairs_down_atlas_coord];
        }

        let tile = tile.unwrap();
        match &tile.specific {
            SpecificTS::Filled => smallvec![self.wall_atlas_coord],
            SpecificTS::Open(OpenType::Hallway) => smallvec![self.hallway_atlas_coord],
            SpecificTS::Open(OpenType::Room) => smallvec![self.room_atlas_coord],
            SpecificTS::Door(state) => {
                let mut door_sprites = smallvec![if state.open {
                    self.open_door_atlas_coord
                } else {
                    self.closed_door_atlas_coord
                }];
                if state.pitoned {
                    door_sprites.push(self.door_pitoned_atlas_coord);
                }
                door_sprites
            }
        }
    }
}

#[godot_api]
impl IControl for FloorLayoutViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            tile_map: None,
            player: None,
            entities: None,

            wall_atlas_coord: -Vector2i::ONE,
            hallway_atlas_coord: -Vector2i::ONE,
            room_atlas_coord: -Vector2i::ONE,
            closed_door_atlas_coord: -Vector2i::ONE,
            open_door_atlas_coord: -Vector2i::ONE,
            door_pitoned_atlas_coord: -Vector2i::ONE,
            stairs_down_atlas_coord: -Vector2i::ONE,
            stairs_up_atlas_coord: -Vector2i::ONE,

            body_entity_scene_path: "".into(),
            body_entity_scene: None,
            campfire_entity_scene_path: "".into(),
            campfire_entity_scene: None,
            gold_entity_scene_path: "".into(),
            gold_entity_scene: None,
            chest_entity_scene_path: "".into(),
            chest_entity_scene: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        let gd_self = self.to_gd();

        self.in_dungeon = Some(walk_parents_for(&gd_self));
        self.in_dungeon.as_mut().unwrap().connect(
            InDungeonViz::UPDATED_STATE_SIGNAL.into(),
            gd_self.callable("_on_in_dungeon_updated"),
        );

        self.body_entity_scene = Some(load(self.body_entity_scene_path.clone()));
        self.campfire_entity_scene = Some(load(self.campfire_entity_scene_path.clone()));
        self.gold_entity_scene = Some(load(self.gold_entity_scene_path.clone()));
        self.chest_entity_scene = Some(load(self.chest_entity_scene_path.clone()));
    }

    fn ready(&mut self) {
        let di_context = DiContext::get_nearest(self.base().clone().upcast()).unwrap();
        let di_context = di_context.bind();

        self.tile_map = Some(di_context.get_registered_node_template::<TileMap>("".into()));
    }
}

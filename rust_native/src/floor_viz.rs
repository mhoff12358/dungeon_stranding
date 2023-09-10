use ds_lib::{
    coord::Coord,
    dungeon_state::tile_state::{OpenType, SpecificTS, TileState},
    game_state::game_state::InDungeon,
};
use godot::{
    engine::{Control, ControlVirtual, TileMap},
    prelude::*,
};

use crate::{in_dungeon_viz::InDungeonViz, tree_utils::walk_parents_for};

#[derive(GodotClass)]
#[class(base=Control)]
pub struct FloorViz {
    in_dungeon: Option<Gd<InDungeonViz>>,

    #[export]
    tile_map: Option<Gd<TileMap>>,
    #[export]
    player: Option<Gd<Node2D>>,

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
    stairs_up_atlas_coord: Vector2i,
    #[export]
    stairs_down_atlas_coord: Vector2i,

    #[base]
    base: Base<Control>,
}

#[godot_api]
impl FloorViz {
    #[func]
    pub fn in_dungeon(&self) -> Gd<InDungeonViz> {
        self.in_dungeon.as_ref().unwrap().share()
    }

    #[func]
    pub fn _on_in_dungeon_updated(&mut self) {
        let in_dungeon = self.in_dungeon();
        let in_dungeon = in_dungeon.bind();
        let in_dungeon = in_dungeon.borrow_in_dungeon();
        let floor = in_dungeon.get_current_floor();
        let bounds = floor.bounds();
        for y in bounds.min.y..=bounds.max.y {
            for x in bounds.min.x..=bounds.max.x {
                let tile_coord = Coord { x, y };
                let atlas_coord = self.get_atlas_coord_for_tile(
                    &in_dungeon,
                    &tile_coord,
                    floor.tiles().get(&tile_coord),
                );
                let tile_map = self.tile_map.as_mut().unwrap();
                tile_map
                    .set_cell_ex(0, Vector2i { x, y })
                    .source_id(0)
                    .atlas_coords(atlas_coord)
                    .done();
            }
        }

        let tile_size = self
            .tile_map
            .as_mut()
            .unwrap()
            .get_tileset()
            .unwrap()
            .get_tile_size();
        let player = self.player.as_mut().unwrap();
        player.set_position(Vector2 {
            x: (in_dungeon.player_position.x * tile_size.x) as f32,
            y: (in_dungeon.player_position.y * tile_size.y) as f32,
        });
    }
}

impl FloorViz {
    fn get_atlas_coord_for_tile(
        &self,
        in_dungeon: &InDungeon,
        coord: &Coord,
        tile: Option<&TileState>,
    ) -> Vector2i {
        if tile.is_none() {
            return -Vector2i::ONE;
        }

        let floor = in_dungeon.get_current_floor();

        if floor.stairs().up == *coord {
            return self.stairs_up_atlas_coord;
        } else if floor.stairs().down == *coord {
            return self.stairs_down_atlas_coord;
        }

        let tile = tile.unwrap();
        match &tile.specific {
            SpecificTS::Filled => self.wall_atlas_coord,
            SpecificTS::Open(OpenType::Hallway) => self.hallway_atlas_coord,
            SpecificTS::Open(OpenType::Room) => self.room_atlas_coord,
            SpecificTS::Door { open } => {
                if *open {
                    self.open_door_atlas_coord
                } else {
                    self.closed_door_atlas_coord
                }
            }
        }
    }
}

#[godot_api]
impl ControlVirtual for FloorViz {
    fn init(base: godot::obj::Base<Self::Base>) -> Self {
        Self {
            in_dungeon: None,
            tile_map: None,
            player: None,

            wall_atlas_coord: -Vector2i::ONE,
            hallway_atlas_coord: -Vector2i::ONE,
            room_atlas_coord: -Vector2i::ONE,
            closed_door_atlas_coord: -Vector2i::ONE,
            open_door_atlas_coord: -Vector2i::ONE,
            stairs_down_atlas_coord: -Vector2i::ONE,
            stairs_up_atlas_coord: -Vector2i::ONE,

            base,
        }
    }

    fn enter_tree(&mut self) {
        self.in_dungeon = Some(walk_parents_for(&self.base));
        self.in_dungeon.as_mut().unwrap().connect(
            "updated_state".into(),
            self.base.callable("_on_in_dungeon_updated"),
        );
    }
}

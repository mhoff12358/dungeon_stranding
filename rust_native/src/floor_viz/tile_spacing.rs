use ds_lib::coord::Coord;
use godot::{engine::TileMap, prelude::*};

pub struct TileSpacing {
    tile_size: Vector2i,
}

impl TileSpacing {
    pub fn new(tile_map: &Gd<TileMap>) -> Self {
        Self {
            tile_size: tile_map.get_tileset().unwrap().get_tile_size(),
        }
    }

    pub fn entity_position(&self, coord: Coord) -> Vector2 {
        (Vector2::new(coord.x as f32, coord.y as f32) + Vector2::new(0.5, 0.5))
            * Vector2::from_vector2i(self.tile_size)
    }

    pub fn tile_size(&self) -> Vector2i {
        self.tile_size
    }
}

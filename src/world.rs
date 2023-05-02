use macroquad::prelude::*;
use macroquad::rand;

use crate::common::*;
use crate::grid::{Grid, Tile};
use crate::particule::*;

#[derive(Debug, Clone)]
pub struct Settings {
    pub selected_particule_type: ParticuleType,
}

#[derive(Debug)]
pub struct World {
    pub particule_size: f32,
    pub grid: Grid,
    pub settings: Settings,
}

impl World {
    pub fn create_particule(&self, particule_type: ParticuleType) -> Particule {
        match particule_type {
            ParticuleType::SAND => Particule {
                size: Some(self.grid.tile_size.to_owned()),
                color: particule_type.clone().get_random_color_varian(),
                ptype: particule_type,
            },
            ParticuleType::WATER => Particule {
                size: Some(self.grid.tile_size.to_owned()),
                color: particule_type.clone().get_random_color_varian(),
                ptype: particule_type,
            },
            ParticuleType::OIL => Particule {
                size: Some(self.grid.tile_size.to_owned()),
                color: particule_type.clone().get_random_color_varian(),
                ptype: particule_type,
            },
            _ => unreachable!(),
        }
    }

    pub fn put_particule(&mut self, particule: Particule, tile: Tile) {
        Grid::put_particule(&mut self.grid.map, particule, &tile);
    }

    pub fn generate_particules(&mut self) -> i32 {
        let count: i32 = self.grid.lines * self.grid.columns / 10;

        for _ in 0..(count - 1) {
            let position = self.fit_position(Position {
                x: rand::gen_range(0.0, screen_width()),
                y: rand::gen_range(0.0, screen_height()),
            });


            let particule = self.create_particule(self.settings.selected_particule_type.clone());

            Grid::put_particule(
                &mut self.grid.map,
                particule,
                &Grid::position_to_tile(self.grid.tile_size.clone(), position),
            );
        }
        count
    }

    pub fn update(&mut self) {
        self.grid.update();
    }

    pub fn draw(&self) {
        //self.grid.draw();
        self.grid.draw_particules();
    }

    pub fn fit_position(&self, position: Position) -> Position {
        let tile = Grid::position_to_tile(self.grid.tile_size.clone(), position.to_owned());

        Position {
            x: tile.x as f32 * self.grid.tile_size.width,
            y: tile.y as f32 * self.grid.tile_size.height,
        }
    }

    pub fn count_particules(&self) -> i32 {
        self.grid.count_particules()
    }
}

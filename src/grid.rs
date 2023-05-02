use std::ops::Add;

use crate::common::*;
use crate::particule::*;
use ::rand::seq::SliceRandom;
use ::rand::thread_rng;
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}
type ParticulesMap = Vec<Vec<Particule>>;

impl Add for Tile {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub lines: i32,
    pub columns: i32,
    pub tile_size: Size,
    pub map: ParticulesMap,
}

impl Grid {
    pub fn new(tile_size: Size) -> Grid {
        let lines = (screen_height() / tile_size.height) as i32;
        let columns = (screen_width() / tile_size.width) as i32;
        Grid {
            lines,
            columns,
            tile_size: tile_size.clone(),
            map: vec![
                vec![
                    Particule {
                        ptype: ParticuleType::EMPTY,
                        size: Some(tile_size.clone()),
                        color: None
                    };
                    lines as usize
                ];
                columns as usize
            ],
        }
    }

    pub fn draw(&self) {
        // a line: always start from x = 0 to x = width and y gap is the tile size
        let mut tile_end: f32 = 0.0;
        for _ in 0..self.lines {
            draw_line(0.0, tile_end, screen_width(), tile_end, 1.0, GRAY);
            tile_end += self.tile_size.height;
        }

        // a column: always start from y = 0 to y = height and x gap is the tile size
        tile_end = 0.0;
        for _ in 0..self.columns {
            draw_line(tile_end, 0.0, tile_end, screen_height(), 1.0, GRAY);
            tile_end += self.tile_size.width;
        }
    }

    pub fn draw_particules(&self) {
        for line in 0..self.lines {
            for col in 0..self.columns {
                let tile = Tile {
                    y: line as i32,
                    x: col as i32,
                };
                match Grid::get_particule_at_tile(&self.map, &tile) {
                    Some(p) => {
                        //p.update_position(Grid::tile_to_position(&self, tile))
                        let position = self.tile_to_position(tile);
                        p.draw(position);
                    }
                    None => {
                        //
                    }
                }
            }
        }
    }

    pub fn out_of_bound_tile(tile: &Tile, map: &ParticulesMap) -> bool {
        let max_tile = Tile {
            x: map.len() as i32,
            y: map[0].len() as i32,
        };

        if tile.y >= max_tile.y || tile.y < 0 || tile.x >= max_tile.x || tile.x < 0 {
            return true;
        }
        false
    }
    pub fn remove_particule_at_tile(map: &mut ParticulesMap, tile: &Tile) {
        map[tile.x as usize][tile.y as usize] = Particule {
            ptype: ParticuleType::EMPTY,
            color: None,
            size: None,
        };
    }

    //pub fn move_particule(&mut self, from_tile: &Tile, to_tile: &Tile) {
    //    match Grid::get_particule_at_tile(&self.map, from_tile) {
    //        Some(mut p) => {
    //            //
    //            //p.update_position(self.tile_to_position(to_tile.clone()));
    //            self.put_particule(p, to_tile);
    //            self.remove_particule_at_tile(from_tile)
    //        }
    //        None => {
    //            //
    //        }
    //    }
    //}

    //pub fn swipe_particules(&mut self, a_tile: &Tile, b_tile: &Tile) {
    //    let a_particule = Grid::get_particule_at_tile(&self.map, &a_tile);
    //    let b_particule = Grid::get_particule_at_tile(&self.map, &b_tile);

    //    self.put_particule(a_particule.unwrap(), b_tile);
    //    self.put_particule(b_particule.unwrap(), a_tile);
    //}

    //pub fn freeze_particule(&mut self, tile: Tile) {
    //    match Grid::get_particule_at_tile(&self.map, &tile) {
    //        Some(mut p) => {
    //            //
    //            self.put_particule(p, &tile);
    //        }
    //        None => {
    //            //
    //        }
    //    }
    //}

    pub fn update(&mut self) {
        for line_idx in (0..self.lines - 1).rev() {
            for col_idx in 0..self.columns - 1 {
                let current_tile = Tile {
                    y: line_idx,
                    x: col_idx,
                };

                match Grid::get_particule_at_tile(&self.map, &current_tile) {
                    None => {
                        // out of bound nothing to do
                    }
                    Some(particule) => update_particule(particule, &current_tile, &mut self.map),
                }
            }
        }
    }

    pub fn position_to_tile(tile_size: Size, position: Position) -> Tile {
        Tile {
            x: (position.x / tile_size.width) as i32,
            y: (position.y / tile_size.height) as i32,
        }
    }

    pub fn tile_to_position(&self, tile: Tile) -> Position {
        Position {
            x: self.tile_size.width * tile.x as f32,
            y: self.tile_size.height * tile.y as f32,
        }
    }

    pub fn get_particule_at_tile(map: &ParticulesMap, tile: &Tile) -> Option<Particule> {
        if tile.x as usize >= map.len() || tile.y as usize >= map[0].len() {
            return None;
        }
        Some(map[tile.x as usize][tile.y as usize].to_owned())
    }

    pub fn put_particule(map: &mut ParticulesMap, particule: Particule, tile: &Tile) {
        if tile.x as usize >= map.len() || tile.y as usize >= map[0].len() {
            return;
        }
        map[tile.x as usize][tile.y as usize] = particule;
    }

    pub fn count_particules(&self) -> i32 {
        let mut count = 0;
        for line in &self.map {
            for p in line {
                match p.ptype {
                    ParticuleType::EMPTY => {
                        //
                    }
                    _ => {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    //pub fn get_neighbours(tile: Tile, map: &ParticulesMap) -> ParticulesMap {
    //    todo!("maybe")
    //}

    fn is_empty(map: &ParticulesMap, tile: &Tile) -> bool {
        match Grid::get_particule_at_tile(map, tile) {
            None => false,
            Some(p) => p.ptype == ParticuleType::EMPTY,
        }
    }
}

fn update_sand_particule(tile: &Tile, map: &mut ParticulesMap) {
    let possible_tiles = vec![
        tile.clone() + Tile { x: 0, y: 1 },
        tile.clone() + Tile { x: -1, y: 1 },
        tile.clone() + Tile { x: 1, y: 1 },
    ];

    let current_particule = Grid::get_particule_at_tile(&map, &tile).unwrap();

    for t in &possible_tiles {
        if Grid::out_of_bound_tile(&t, &map) {
            continue;
        }

        match Grid::get_particule_at_tile(&map, &t) {
            None => {
                continue;
            }
            Some(p) => match p.ptype {
                ParticuleType::WATER | ParticuleType::EMPTY => {
                    // do move
                    Grid::put_particule(map, current_particule, t);
                    Grid::remove_particule_at_tile(map, tile);
                    return;
                }
                _ => continue,
            },
        }
    }
}

fn update_water_particule(tile: &Tile, map: &mut ParticulesMap) {
    let below_tile = tile.clone() + Tile { x: 0, y: 1 };

    let current_particule = Grid::get_particule_at_tile(&map, &tile).unwrap();

    if !Grid::out_of_bound_tile(&below_tile, &map) && Grid::is_empty(&map, &below_tile) {
        match Grid::get_particule_at_tile(&map, &below_tile) {
            None => {
                // out of bounds
            }
            Some(p) => {
                match p.ptype {
                    ParticuleType::EMPTY => {
                        Grid::put_particule(map, current_particule, &below_tile);
                        Grid::remove_particule_at_tile(map, tile);
                        return;
                    }
                    _ => {
                        // not empty
                    }
                }
            }
        }
    }

    let mut possible_tiles = vec![
        tile.clone() + Tile { x: -1, y: 1 },
        tile.clone() + Tile { x: 1, y: 1 },
    ];

    possible_tiles.shuffle(&mut thread_rng());

    for t in &possible_tiles {
        //println!("{:?}", t);
        if !Grid::out_of_bound_tile(&t, &map) && Grid::is_empty(&map, t) {
            match Grid::get_particule_at_tile(&map, &below_tile) {
                None => {
                    continue;
                }
                Some(p) => {
                    match p.ptype {
                        ParticuleType::EMPTY => {
                            Grid::put_particule(map, current_particule, t);
                            Grid::remove_particule_at_tile(map, tile);
                            return;
                        }
                        _ => {
                            // not empty
                        }
                    }
                }
            }
        }
    }

    for o in 2..map.len() - 1 {
        let t = tile.clone() + Tile { x: o as i32, y: 1 };

        if Grid::out_of_bound_tile(&t, map) {
            continue;
        }

        match Grid::get_particule_at_tile(map, &t) {
            None => continue,
            Some(p) => {
                if p.ptype == ParticuleType::SAND {
                    continue;
                }
            }
        };

        let t = tile.clone()
            + Tile {
                x: -(o as i32),
                y: 1,
            };

        if Grid::out_of_bound_tile(&t, map) {
            continue;
        }

        match Grid::get_particule_at_tile(map, &t) {
            None => continue,
            Some(p) => match p.ptype {
                ParticuleType::EMPTY => {
                    Grid::put_particule(map, current_particule, &t);
                    Grid::remove_particule_at_tile(map, tile);
                    return;
                }
                _ => continue,
            },
        };
    }
}

fn update_particule(particule: Particule, tile: &Tile, map: &mut ParticulesMap) {
    match particule.ptype {
        ParticuleType::EMPTY => { /* */ }
        ParticuleType::SAND => update_sand_particule(tile, map),
        ParticuleType::WATER => update_water_particule(tile, map),
        _ => unreachable!(),
    }
}
